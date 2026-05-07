use std::time::SystemTime;
use influx::model::current::{Compass16, CurrentWeather};
use model::http::current::Current;

const MB_TO_MMHG: f32 = 0.750062;

pub fn from_current_weather_response(current: Current) -> influx::error::Result<CurrentWeather> {
    let wind_dir = Compass16::from_string_ref(current.wind_dir.as_str())?;

    Ok(CurrentWeather {
        id: uuid::Uuid::new_v4().to_string(),
        time: SystemTime::now().into(),
        last_updated_epoch: current.last_updated_epoch,
        last_updated: current.last_updated,
        temp_c: current.temp_c,
        is_day: current.is_day,
        // Unique id of current weather condition
        condition_id: current.condition.map(|c| c.code),
        wind_kph: current.wind_kph,
        wind_degree: current.wind_degree,
        // 16 point compass values. They are N, NNE, NE(45 degrees), ENE(67.5 degrees), E(90 degrees),
        // ESE(112.5 degrees), SE(135 degrees), SSE(157.5 degrees), S(180 degrees), SSW(202.5 degrees),
        // SW(225 degrees), WSW(247.5 degrees), W(270 degrees), WNW(292.5 degrees), NW(315 degrees)
        wind_dir,
        // Pressure in millibars; mmHg value = mbar value x 0.750062
        pressure_mb: current.pressure_mb,
        // Pressure in mmHg
        pressure_mmhg: current.pressure_mb * MB_TO_MMHG,
        precip_mm: current.precip_mm,
        // Humidity as percentage
        humidity: current.humidity,
        // Cloud cover as percentage
        cloud_percentage: current.cloud,
        feelslike_c: current.feelslike_c,
        vis_km: current.vis_km,
        uv: current.uv,
        gust_kph: current.gust_kph,
    })
}
