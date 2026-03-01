#![allow(dead_code)]
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Forecast {
    forecastday: Vec<ForecastDay>
}

#[derive(Deserialize, Debug)]
pub struct ForecastDay {
    date: String,
    //date_epoch: i32,
    day: DayElement,
    astro: AstroElement,
    air_quality: Option<AqiElement>,
    hour: Vec<HourElement>,
}

#[derive(Debug, Deserialize)]
pub struct DayElement {
    maxtemp_c: f32,
    //maxtemp_f: f32,
    mintemp_c: f32,
    //mintemp_f: f32,
    avgtemp_c: f32,
    //avgtemp_f: f32,
    //maxwind_mph: f32,
    maxwind_kph: f32,
    totalprecip_mm: f32,
    //totalprecip_in: f32,
    avgvis_km: f32,
    //avgvis_miles: f32,
    avghumidity: f32,
    daily_will_it_rain: i32,
    daily_chance_of_rain: f32,
    daily_will_it_snow: i32,
    daily_chance_of_snow: f32,
    condition: Option<DayCondition>,
    uv: f32,
}

#[derive(Debug, Deserialize)]
pub struct DayCondition {
    text: String,
    //icon: String,
    //code: i32,
}

#[derive(Debug, Deserialize)]
struct AstroElement {
    sunrise: String,
    sunset: String,
    //moonrise: String,
    //moonset: String,
    moon_phase: String,
    moon_illumination: i32,
}

#[derive(Debug, Deserialize)]
struct AqiElement {
    co: f32,
    no2: f32,
    o3: f32,
    so2: f32,
    pm25: f32,
    pm10: f32,
    us_epa_index: i32,
    gb_defra_index: i32,
}

#[derive(Debug, Deserialize)]
struct HourElement {
    //time_epoch: i32,
    time: String,
    temp_c: f32,
    //temp_f: f32,
    //is_day: i32,
    condition: Option<ForecastCondition>,
    //wind_mph: f32,
    wind_kph: f32,
    wind_degree: f32,
    wind_dir: String,
    pressure_mb: f32,
    //pressure_in: f32,
    precip_mm: f32,
    //precip_in: f32,
    humidity: f32,
    cloud: f32,
    feelslike_c: f32,
    //feelslike_f: f32,
    windchill_c: f32,
    //windchill_f: f32,
    heatindex_c: f32,
    //heatindex_f: f32,
    dewpoint_c: f32,
    //dewpoint_f: f32,
    will_it_rain: i32,
    chance_of_rain: f32,
    will_it_snow: i32,
    chance_of_snow: f32,
    vis_km: f32,
    //vis_miles: f32,
    //gust_mph: f32,
    gust_kph: f32,
    uv: f32,
}

#[derive(Debug, Deserialize)]
struct ForecastCondition {
    text: String,
    icon: String,
    code: i32,
}
