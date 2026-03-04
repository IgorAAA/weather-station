use crate::config::AppConfig;
use http_client::{HttpClient, WeatherClient};
use influx::client::InfluxClient;
use influx::model::current::{Compass16, CurrentWeather};
use influx::{LogWriter, WeatherWriter};
use metrics::run_metrics_server;
use model::http::current::{Current, CurrentWeatherResponse};
use std::time::{Duration, SystemTime};
use twelf::reexports::log::{debug, error};

mod config;
mod metrics;

#[tokio::main]
async fn main() {
    env_logger::init();

    metrics::spawn_host_metrics_updater();

    tokio::spawn(async move {
        run_metrics_server().await;
    });

    let config = AppConfig::build().expect("Cannot build config");

    debug!("App config file found: {:#?}", config);

    let weather_api_config = config.weather_api_config;
    let timeout = weather_api_config.timeout;

    let http_client = WeatherClient::new(weather_api_config).expect("Cannot build http client"); // Stop the app if http client is not build

    let influxdb_config = config.influx_db_config;
    let influxdb_client = InfluxClient::new(influxdb_config);
    let influx_writer = WeatherWriter::LogCurrentWeather(LogWriter);

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

        tokio::time::sleep(Duration::from_secs(timeout)).await;
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
