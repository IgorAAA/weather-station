use crate::config::AppConfig;
use http_client::parse_url;
use twelf::reexports::log::{debug, info};

mod config;

#[tokio::main]
async fn main() {
    env_logger::init();

    let config = AppConfig::build().expect("Cannot build config");

    debug!("App config file found: {:#?}", config);

    let weather_api_config = config.weather_api_config;

    // Preparing required url to be sent to weather api
    let parsed_url = parse_url(weather_api_config).expect("Cannot parse url"); // Stop the app if the url is incorrect

    info!("Parsed url: {:#?}", parsed_url);
}
