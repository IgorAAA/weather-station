use crate::error::{Error, Result};
use chrono::{DateTime, Utc};
use influxdb::{InfluxDbWriteable, Type};
use std::string::ToString;
use strum_macros::Display;

/// Compass with 16 directions
#[allow(non_snake_case)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Display, PartialEq)]
pub enum Compass16 {
    /// 0 degrees
    N,
    /// 22.5 degrees
    NNE,
    /// 45 degrees
    NE,
    /// 67.5 degrees
    ENE,
    /// 90 degrees
    E,
    /// 112.5 degrees
    ESE,
    /// 135 degrees
    SE,
    /// 157.5 degrees
    SSE,
    /// 180 degrees
    S,
    /// 202.5 degrees
    SSW,
    /// 225 degrees
    SW,
    /// 247.5 degrees
    WSW,
    /// 270 degrees
    W,
    /// 292.5 degrees
    WNW,
    /// 315 degrees
    NW,
    /// 335.5 degrees
    NNW,
}

impl Compass16 {
    /// Safely converts a string into a value of Compass16
    /// * `value` - String to be converted to Compass16
    pub fn from_string_ref(value: &str) -> Result<Self> {
        // TODO use strum instead
        match value.to_lowercase().as_str() {
            "n" => Ok(Compass16::N),
            "nne" => Ok(Compass16::NNE),
            "ne" => Ok(Compass16::NE),
            "ene" => Ok(Compass16::ENE),
            "e" => Ok(Compass16::E),
            "ese" => Ok(Compass16::ESE),
            "se" => Ok(Compass16::SE),
            "sse" => Ok(Compass16::SSE),
            "s" => Ok(Compass16::S),
            "ssw" => Ok(Compass16::SSW),
            "sw" => Ok(Compass16::SW),
            "wsw" => Ok(Compass16::WSW),
            "w" => Ok(Compass16::W),
            "wnw" => Ok(Compass16::WNW),
            "nw" => Ok(Compass16::NW),
            "nnw" => Ok(Compass16::NNW),
            _ => Err(Error::CompassError(format!(
                "Cannot recognize wind direction from value {}",
                value
            ))),
        }
    }
}

/// Represents current weather record in influx
#[derive(InfluxDbWriteable, Debug)]
pub struct CurrentWeather {
    /// ID of the record
    pub id: String,
    /// Timestamp in UTC
    pub time: DateTime<Utc>,
    /// Last updated epoch
    pub last_updated_epoch: i32,
    /// Last updated
    pub last_updated: String,
    /// Temperature in Celsius
    pub temp_c: f32,
    /// Indicates whether it's a day or a night
    pub is_day: i32,
    /// Unique id of current weather condition
    pub condition_id: Option<i32>,
    /// Wind speed in km / hour
    pub wind_kph: f32,
    /// Wind direction in degrees
    pub wind_degree: f32,
    /// 16 point compass values - See Compass16
    pub wind_dir: Compass16,
    /// Pressure in millibars; mmHg value = mbar value x 0.750062
    pub pressure_mb: f32,
    /// Pressure in inches
    pub pressure_mmhg: f32,
    /// Precipitation in mm
    pub precip_mm: f32,
    /// Humidity as percentage
    pub humidity: f32,
    /// Cloud cover as percentage
    pub cloud_percentage: f32,
    /// The temperature 'feels like'
    pub feelslike_c: f32,
    /// Visibility in km
    pub vis_km: f32,
    /// UV radiation
    pub uv: f32,
    /// Gust of the wind in km / hour
    pub gust_kph: f32,
}

impl From<Compass16> for Type {
    /// Converts value from Compass16 to Influx's Type
    /// Required as Influx has pre-defined converters only for simple types
    /// * value - Compass16 object
    fn from(value: Compass16) -> Self {
        Type::Text(value.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error;

    #[test]
    fn display_test() {
        let ene = super::Compass16::ENE;

        assert_eq!(ene.to_string(), String::from("ENE"));
    }

    #[test]
    fn compass_from_string_ref_valid() {
        // check several variants (case-insensitive)
        assert_eq!(Compass16::from_string_ref("N").unwrap(), Compass16::N);
        assert_eq!(Compass16::from_string_ref("nne").unwrap(), Compass16::NNE);
        assert_eq!(Compass16::from_string_ref("Ne").unwrap(), Compass16::NE);
        assert_eq!(Compass16::from_string_ref("ENE").unwrap(), Compass16::ENE);
        assert_eq!(Compass16::from_string_ref("w").unwrap(), Compass16::W);
        assert_eq!(Compass16::from_string_ref("NnW").unwrap(), Compass16::NNW);
    }

    #[test]
    fn compass_from_string_ref_invalid() {
        let err = Compass16::from_string_ref("not-a-direction").unwrap_err();
        match err {
            error::Error::CompassError(s) => {
                assert!(s.contains("Cannot recognize wind direction"));
            }
            _ => panic!("expected CompassError variant"),
        }
    }
}
