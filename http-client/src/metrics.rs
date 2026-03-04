use once_cell::sync::Lazy;
use prometheus::{HistogramVec, IntCounterVec, register_histogram_vec, register_int_counter_vec};

pub static HTTP_CLIENT_REQUESTS_TOTAL: Lazy<IntCounterVec> = Lazy::new(|| {
    register_int_counter_vec!(
        "http_client_requests_total",
        "Total number of HTTP client requests",
        &["method", "host", "status"]
    )
    .expect("metrics can be registered")
});

pub static HTTP_CLIENT_REQUESTS_ERRORS: Lazy<IntCounterVec> = Lazy::new(|| {
    register_int_counter_vec!(
        "http_client_requests_errors_total",
        "Total outbound HTTP request errors",
        &["method", "host", "error_type"]
    )
    .expect("metrics can be registered")
});

pub static HTTP_CLIENT_REQUESTS_DURATION_SECONDS: Lazy<HistogramVec> = Lazy::new(|| {
    register_histogram_vec!(
        "http_client_requests_duration_seconds",
        "Outbound HTTP request duration in seconds",
        &["method", "host", "status"],
        vec![0.005, 0.01, 0.025, 0.05, 0.075, 1.0, 2.5, 5.0, 10.0]
    )
    .expect("metrics can be registered")
});
