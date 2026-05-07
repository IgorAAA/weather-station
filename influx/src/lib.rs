use crate::client::InfluxClient;
use crate::model::current::CurrentWeather;
use log::{error, info};

pub mod client;
pub mod config;
pub mod error;
pub mod model;

pub enum WeatherWriter {
    // Real writer to influxdb
    InfluxCurrentWeather(InfluxWriter),
    // Log writer -- for demo, etc.
    LogCurrentWeather(LogWriter),
}

pub struct InfluxWriter;

impl InfluxWriter {
    async fn write_current_weather(&self, client: &InfluxClient, current_weather: CurrentWeather) {
        match client
            .write_to_influx_db("current_weather_test", current_weather)
            .await
        {
            Ok(_) => info!("InfluxDB write successful"),
            Err(e) => error!("InfluxDB write error: {:#?}", e),
        }
    }
}

pub struct LogWriter;

impl LogWriter {
    async fn write_current_weather(&self, _client: &InfluxClient, current_weather: CurrentWeather) {
        log::info!(
            "Current Weather to write to influxdb: {:#?}",
            current_weather
        );
    }
}

impl WeatherWriter {
    pub async fn write_current_weather(
        &self,
        client: &InfluxClient,
        current_weather: CurrentWeather,
    ) {
        match self {
            WeatherWriter::InfluxCurrentWeather(writer) => {
                writer.write_current_weather(client, current_weather).await
            }
            WeatherWriter::LogCurrentWeather(writer) => {
                writer.write_current_weather(client, current_weather).await
            }
        }
    }
}
