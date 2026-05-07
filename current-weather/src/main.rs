use crate::config::{AppConfig, WriterKind};
use http_client::{HttpClient, WeatherClient};
use influx::client::InfluxClient;
use influx::model::current::{Compass16, CurrentWeather};
use influx::{InfluxWriter, LogWriter, WeatherWriter};
use metrics::run_metrics_server;
use model::http::current::{Current, CurrentWeatherResponse};
use std::time::{Duration, SystemTime};
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
        shutdown_rx.clone(),
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
    use tokio::signal::unix::{SignalKind, signal};
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

pub fn from_current_weather_response(current: Current) -> influx::error::Result<CurrentWeather> {
    let wind_dir = Compass16::from_string_ref(current.wind_dir.as_str())?;

    Ok(CurrentWeather {
        id: uuid::Uuid::new_v4().to_string(),
        time: SystemTime::now().into(),
        last_updated_epoch: current.last_updated_epoch,
        last_updated: current.last_updated,
        temp_c: current.temp_c,
        is_day: current.is_day,
        // Unique id of current weather condition
        condition_id: current.condition.map(|c| c.code),
        wind_kph: current.wind_kph,
        wind_degree: current.wind_degree,
        // 16 point compass values. They are N, NNE, NE(45 degrees), ENE(67.5 degrees), E(90 degrees),
        // ESE(112.5 degrees), SE(135 degrees), SSE(157.5 degrees), S(180 degrees), SSW(202.5 degrees),
        // SW(225 degrees), WSW(247.5 degrees), W(270 degrees), WNW(292.5 degrees), NW(315 degrees)
        wind_dir,
        // Pressure in millibars; mmHg value = mbar value x 0.750062
        pressure_mb: current.pressure_mb,
        // Pressure in inches
        pressure_mmhg: current.pressure_mb * 0.750062,
        precip_mm: current.precip_mm,
        // Humidity as percentage
        humidity: current.humidity,
        // Cloud cover as percentage
        cloud_percentage: current.cloud,
        feelslike_c: current.feelslike_c,
        vis_km: current.vis_km,
        uv: current.uv,
        gust_kph: current.gust_kph,
    })
}
