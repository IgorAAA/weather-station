use chrono::Utc;
use influx::model::current::{Compass16, CurrentWeather};
use influxdb::{InfluxDbWriteable, Type};

#[test]
fn current_weather_into_query_contains_expected_fields() {
    // Build a sample CurrentWeather record
    let sample = CurrentWeather {
        id: "test-id".to_string(),
        time: Utc::now(),
        last_updated_epoch: 1,
        last_updated: "2026-01-22 00:00".to_string(),
        temp_c: 12.34,
        is_day: 1,
        condition_id: Some(100),
        wind_kph: 5.5,
        wind_degree: 90.0,
        wind_dir: Compass16::NE,
        pressure_mb: 1013.25,
        pressure_mmhg: 1013.25 * 0.750062,
        precip_mm: 0.0,
        humidity: 80.0,
        cloud_percentage: 20.0,
        feelslike_c: 11.0,
        vis_km: 10.0,
        uv: 0.0,
        gust_kph: 7.2,
    };

    // Generate the InfluxDB line-protocol query for this payload.
    // The derive provided by `influxdb` crate exposes `into_query`.
    let line = sample.into_query("measurement_name");
    let line = format!("{:?}", line);

    // Basic sanity checks: measurement name present and several fields included
    assert!(
        line.contains("measurement_name"),
        "line protocol should contain measurement name"
    );
    assert!(
        line.contains("temp_c"),
        "line protocol should contain temp_c field"
    );
    assert!(
        line.contains("wind_kph"),
        "line protocol should contain wind_kph field"
    );
    // wind_dir is converted to text via From<Compass16> for Type -> should include NE
    assert!(
        line.contains("NE") || line.contains("wind_dir"),
        "line protocol should include compass text or wind_dir tag"
    );
    // id should also be present somewhere (as tag or field depending on derive)
    assert!(
        line.contains("test-id") || line.contains("id="),
        "line protocol should include id"
    );
}
