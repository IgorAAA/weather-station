pub mod config;
pub mod error;

use crate::config::WeatherApiConfig;
use error::Result;
use url::Url;

pub fn parse_url(config: WeatherApiConfig) -> Result<Url> {
    let base_url = config.base_url;

    let current_uri = format!("{}/current.json", base_url);

    let coords = config.coords.as_str();

    let weather_api_key = config.weather_api_key.as_str();

    let params = vec![("key", weather_api_key), ("q", coords)];
    let result = Url::parse_with_params(&current_uri, params)?;
    Ok(result)
}
