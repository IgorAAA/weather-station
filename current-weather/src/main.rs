use crate::config::{AppConfig, WriterKind};
use current_weather::from_current_weather_response;
use http_client::{HttpClient, WeatherClient};
use influx::client::InfluxClient;
use influx::{InfluxWriter, LogWriter, WeatherWriter};
use metrics::run_metrics_server;
use model::http::current::CurrentWeatherResponse;
use std::time::Duration;
use tokio::signal;
use tokio::sync::watch;
use tracing::{debug, error, info};
use tracing_subscriber::EnvFilter;

mod config;
mod metrics;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let config = AppConfig::build().expect("Cannot build config");

    debug!("App config file found: {:#?}", config);

    let (shutdown_tx, shutdown_rx) = watch::channel(false);

    let mut host_metrics_handle = metrics::spawn_host_metrics_updater(shutdown_rx.clone());

    let metrics_bind_addr = config.metrics_bind_addr.clone();
    let metrics_shutdown_rx = shutdown_rx.clone();
    let mut metrics_server_handle = tokio::spawn(async move {
        run_metrics_server(&metrics_bind_addr, metrics_shutdown_rx).await
    });

    let weather_api_config = config.weather_api_config;
    let poll_interval_secs = weather_api_config.poll_interval_secs;
    let http_client = WeatherClient::new(weather_api_config).expect("Cannot build http client");
    let influxdb_client = InfluxClient::new(config.influx_db_config);
    let influx_writer = match config.writer {
        WriterKind::Influx => WeatherWriter::InfluxCurrentWeather(InfluxWriter),
        WriterKind::Log => WeatherWriter::LogCurrentWeather(LogWriter),
    };

    let mut poll_handle = tokio::spawn(run_poll_loop(
        http_client,
        influx_writer,
        influxdb_client,
        poll_interval_secs,
        shutdown_rx,
    ));

    tokio::select! {
        _ = signal::ctrl_c() => info!("SIGINT received, shutting down"),
        _ = wait_sigterm() => info!("SIGTERM received, shutting down"),
        res = &mut poll_handle => error!("polling task exited unexpectedly: {:?}", res),
        res = &mut metrics_server_handle => error!("metrics server exited: {:?}", res),
        res = &mut host_metrics_handle => error!("host metrics updater exited unexpectedly: {:?}", res),
    }

    let _ = shutdown_tx.send(true);

    let _ = tokio::join!(poll_handle, metrics_server_handle, host_metrics_handle);

    info!("Shutdown complete");
}

#[cfg(unix)]
async fn wait_sigterm() {
    use tokio::signal::unix::{signal, SignalKind};
    match signal(SignalKind::terminate()) {
        Ok(mut s) => {
            let _ = s.recv().await;
        }
        Err(e) => {
            error!("Failed to install SIGTERM handler: {}", e);
            std::future::pending::<()>().await;
        }
    }
}

#[cfg(not(unix))]
async fn wait_sigterm() {
    std::future::pending::<()>().await;
}

async fn run_poll_loop(
    http_client: WeatherClient,
    influx_writer: WeatherWriter,
    influxdb_client: InfluxClient,
    poll_interval_secs: u64,
    mut shutdown: watch::Receiver<bool>,
) {
    loop {
        let response = http_client
            .weather_response::<CurrentWeatherResponse>()
            .await;

        match response {
            Ok(response) => {
                let current_weather = from_current_weather_response(response.current);
                match current_weather {
                    Ok(current_weather) => {
                        influx_writer
                            .write_current_weather(&influxdb_client, current_weather)
                            .await
                    }
                    Err(error) => error!("Incorrect weather params: {:#?}", error),
                }
            }
            Err(error) => {
                error!("Error received from Weather API: {:#?}", error)
            }
        };

        tokio::select! {
            _ = tokio::time::sleep(Duration::from_secs(poll_interval_secs)) => {}
            _ = shutdown.changed() => {
                info!("Polling loop shutting down");
                return;
            }
        }
    }
}

