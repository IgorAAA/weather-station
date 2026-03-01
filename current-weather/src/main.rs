use crate::config::AppConfig;
use http_client::{HttpClient, WeatherClient};
use model::http::current::{Current, CurrentWeatherResponse};
use std::time::Duration;
use twelf::reexports::log::{debug, error, info};

mod config;

#[tokio::main]
async fn main() {
    env_logger::init();

    let config = AppConfig::build().expect("Cannot build config");

    debug!("App config file found: {:#?}", config);

    let weather_api_config = config.weather_api_config;
    let timeout = weather_api_config.timeout;

    let http_client = WeatherClient::new(weather_api_config).expect("Cannot build http client"); // Stop the app if http client is not build

    loop {
        let response = http_client
            .weather_response::<CurrentWeatherResponse>()
            .await;

        match response {
            Ok(response) => {
                let current = response.current;
                write_current_weather_to_influx(current).await;
            }
            Err(error) => {
                error!("Error received from Weather API: {:#?}", error);
            }
        }

        tokio::time::sleep(Duration::from_secs(timeout)).await;
    }
}

async fn write_current_weather_to_influx(current: Current) {
    info!("Current weather response: {:#?}", current);
    // TODO Add influx client
}
