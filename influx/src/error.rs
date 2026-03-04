use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Error when converting sting to compass {0}")]
    CompassError(String),
    #[error("InfluxDB error: {0}")]
    InfluxError(#[from] influxdb::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
