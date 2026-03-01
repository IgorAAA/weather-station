#![allow(dead_code)]
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Current {
    // Local time when the real time data was updated in unix time.
    pub last_updated_epoch: i32,
    // Local time when the real time data was updated.
    pub last_updated: String,
    pub temp_c: f32,
    pub temp_f: f32,
    // 1 = Yes 0 = No; whether to show day condition icon or night icon
    pub is_day: i32,
    pub condition: Option<CurrentCondition>,
    pub wind_mph: f32,
    pub wind_kph: f32,
    pub wind_degree: f32,
    pub wind_dir: String,
    pub pressure_mb: f32,
    pub pressure_in: f32,
    pub precip_mm: f32,
    pub precip_in: f32,
    pub humidity: f32,
    pub cloud: f32,
    pub feelslike_c: f32,
    pub feelslike_f: f32,
    pub vis_km: f32,
    pub vis_miles: f32,
    // UV Index
    pub uv: f32,
    pub gust_mph: f32,
    pub gust_kph: f32,
    pub air_quality: Option<CurrentAirQuality>,
}

#[derive(Deserialize, Debug)]
pub struct CurrentCondition {
    pub text: String,
    pub icon: String,
    pub code: i32,
}

#[derive(Deserialize, Debug)]
pub struct CurrentAirQuality {
    co: f32,
    no2: f32,
    o3: f32,
    so2: f32,
    pm25: f32,
    pm10: f32,
    us_epa_index: i32,
    gb_defra_index: i32,
}
