use http_client::config::WeatherApiConfig;
use influx::config::DbConfig;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use twelf::{Error, Layer, config};

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WriterKind {
    #[default]
    Log,
    Influx,
}

#[config]
#[derive(Debug, Serialize)]
pub struct AppConfig {
    pub influx_db_config: DbConfig,
    pub weather_api_config: WeatherApiConfig,
    pub metrics_bind_addr: String,
    #[serde(default)]
    pub writer: WriterKind,
}

impl AppConfig {
    pub fn build() -> Result<AppConfig, Error> {
        AppConfig::with_layers(&[
            Layer::Toml("./config/app_config.toml".into()),
            Layer::Env(Some("INFLUXDB_TOKEN".to_string())),
            Layer::Env(Some("WEATHER_API_KEY".to_string())),
            Layer::Env(Some("COORDS".to_string())),
        ])
    }
}
