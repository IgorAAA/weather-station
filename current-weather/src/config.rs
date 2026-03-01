use influx::config::DbConfig;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use twelf::{Error, Layer, config};

#[config]
#[derive(Debug, Serialize)]
pub struct AppConfig {
    pub influx_db_config: DbConfig,
    pub weather_api_config: WeatherApiConfig,
}

impl AppConfig {
    pub fn build() -> Result<AppConfig, Error> {
        AppConfig::with_layers(&[
            Layer::Toml("./config/app_config.toml".into()),
            Layer::Env(Some("INFLUXDB_TOKEN".to_string())),
            Layer::Env(Some("INFLUXDB_ID".to_string())),
            Layer::Env(Some("WEATHER_API_KEY".to_string())),
            Layer::Env(Some("COORDS".to_string())),
        ])
    }
}

#[derive(Serialize, Deserialize)]
pub struct WeatherApiConfig {
    pub base_url: String,
    pub coords: String,
    pub weather_api_key: String,
    pub timeout: u32,
}

impl Debug for WeatherApiConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WeatherApiConfig")
            .field("base_url", &self.base_url)
            .field("coords", &self.coords)
            .field("weather_api_key", &"*************")
            .field("timeout", &self.timeout)
            .finish()
    }
}
