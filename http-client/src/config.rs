use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Serialize, Deserialize)]
pub struct WeatherApiConfig {
    pub base_url: String,
    pub coords: String,
    pub weather_api_key: String,
    pub timeout: u64,
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
