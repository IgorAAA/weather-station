use crate::config::AppConfig;
use twelf::reexports::log::info;

mod config;

#[tokio::main]
async fn main() {
    env_logger::init();

    let config = AppConfig::build().expect("Cannot build config");

    info!("App config file found: {:#?}", config);
}
