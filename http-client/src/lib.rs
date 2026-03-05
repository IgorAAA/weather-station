pub mod config;
pub mod error;
mod metrics;

use crate::config::WeatherApiConfig;
use async_trait::async_trait;
use error::Result;
use log::debug;
use metrics::*;
use serde::Deserialize;
use std::time::Instant;
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
    host: String,
}

impl WeatherClient {
    pub fn new(config: WeatherApiConfig) -> Result<Self> {
        let client = reqwest::Client::builder().build()?;
        let url = UrlParser.parse_url(&config, None)?;
        let host = config.host.clone();
        Ok(Self { client, url, host })
    }
}

impl HttpClient for WeatherClient {
    async fn weather_response<Resp: for<'a> Deserialize<'a>>(&self) -> Result<Resp> {
        let started = Instant::now();

        let resp = self.client.get(self.url.clone()).send().await;

        let elapsed = started.elapsed().as_secs_f64();

        let status_label = match &resp {
            Ok(resp) => {
                let status = resp.status().as_u16();
                &status.to_string()
            } //.to_string(),
            Err(err) if err.is_timeout() => "timeout", //.to_string(),
            Err(err) if err.is_connect() => "connect_error", //.to_string(),
            Err(_) => "error",                         //.to_string(),
        };

        let method_label = "GET"; //.to_string();

        HTTP_CLIENT_REQUESTS_TOTAL
            .with_label_values(&[method_label, self.host.as_str(), status_label])
            .inc();

        HTTP_CLIENT_REQUESTS_DURATION_SECONDS
            .with_label_values(&[method_label, self.host.as_str(), status_label])
            .observe(elapsed);

        let text = resp?.text().await?;
        debug!("Response: {}", text);

        let resp = serde_json::from_str::<Resp>(&text)?;
        Ok(resp)
    }
}

struct UrlParser;

impl UrlParser {
    fn parse_url(
        &self,
        config: &WeatherApiConfig,
        additional_params: Option<Vec<(&str, &str)>>,
    ) -> Result<Url> {
        let current_url = format!("{}://{}/v1/current.json", config.scheme, config.host);

        let coords = config.coords.as_str();

        let weather_api_key = config.weather_api_key.as_str();

        let mut params = vec![("key", weather_api_key), ("q", coords)];

        if let Some(mut par) = additional_params {
            params.append(&mut par);
        }

        let result = Url::parse_with_params(&current_url, params)?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_parse_url_valid() {
        let config: WeatherApiConfig = WeatherApiConfig {
            scheme: "http".to_string(),
            host: "api.weather.com".to_string(),
            coords: "New York".to_string(),
            weather_api_key: "abc123".to_string(),
            timeout: 0,
        };

        let url_result = UrlParser.parse_url(&config, Some(vec![("lang", "en")]));
        assert!(url_result.is_ok());

        let expected_url = "http://api.weather.com/v1/current.json?key=abc123&q=New+York&lang=en";
        assert_eq!(url_result.unwrap().to_string(), expected_url);
    }
}
