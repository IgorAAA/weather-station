pub mod config;
pub mod error;

use crate::config::WeatherApiConfig;
use async_trait::async_trait;
use error::Result;
use log::debug;
use serde::Deserialize;
use url::Url;

#[async_trait]
pub trait HttpClient {
    fn weather_response<Resp: for<'a> Deserialize<'a>>(
        &self,
    ) -> impl std::future::Future<Output = Result<Resp>>;
}

pub struct WeatherClient {
    client: reqwest::Client,
    url: Url,
}

impl WeatherClient {
    pub fn new(config: WeatherApiConfig) -> Result<Self> {
        let client = reqwest::Client::builder().build()?;
        let url = UrlParser.parse_url(config)?;
        Ok(Self { client, url })
    }
}

impl HttpClient for WeatherClient {
    async fn weather_response<Resp: for<'a> Deserialize<'a>>(&self) -> Result<Resp> {
        let resp = self.client.get(self.url.as_str()).send().await?;
        debug!("Response status: {}", resp.status());

        let text = resp.text().await?;
        debug!("Response: {}", text);

        let resp = serde_json::from_str::<Resp>(&text)?;
        Ok(resp)
    }
}

struct UrlParser;

impl UrlParser {
    fn parse_url(&self, config: WeatherApiConfig) -> Result<Url> {
        let base_url = config.base_url;

        let current_uri = format!("{}/current.json", base_url);

        let coords = config.coords.as_str();

        let weather_api_key = config.weather_api_key.as_str();

        let params = vec![("key", weather_api_key), ("q", coords)];
        let result = Url::parse_with_params(&current_uri, params)?;
        Ok(result)
    }
}
