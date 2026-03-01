use strum_macros::Display;
use thiserror::Error;

#[derive(Debug, Error, Display)]
pub enum WeatherResponseError {
    Request(#[from] reqwest::Error),
    Json(#[from] serde_json::Error),
    Parse(#[from] url::ParseError),
}

pub type Result<T> = std::result::Result<T, WeatherResponseError>;
