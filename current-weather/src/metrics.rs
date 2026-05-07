use axum::Router;
use axum::body::Body;
use axum::http::{HeaderValue, Response, StatusCode, header};
use axum::routing::get;
use once_cell::sync::Lazy;
use prometheus::{Encoder, Gauge, TextEncoder, register_gauge};
use std::sync::Mutex;
use std::time::Duration;
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};

pub static HOST_CPU_USAGE: Lazy<Gauge> = Lazy::new(|| {
    register_gauge!("host_cpu_usage_percent", "Total host CPU usage percent")
        .expect("metrics can be registered")
});

pub static HOST_MEM_TOTAL_BYTES: Lazy<Gauge> = Lazy::new(|| {
    register_gauge!("host_memory_total_bytes", "Total host memory in bytes")
        .expect("metrics can be registered")
});

pub static HOST_MEM_USED_BYTES: Lazy<Gauge> = Lazy::new(|| {
    register_gauge!("host_memory_used_bytes", "Used host memory in bytes")
        .expect("metrics can be registered")
});

static SYS: Lazy<Mutex<System>> = Lazy::new(|| {
    let refresh = RefreshKind::new()
        .with_cpu(CpuRefreshKind::everything())
        .with_memory(MemoryRefreshKind::everything());
    Mutex::new(System::new_with_specifics(refresh))
});

pub fn spawn_host_metrics_updater() {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(5));
        loop {
            interval.tick().await;

            let mut sys = SYS.lock().unwrap();
            sys.refresh_cpu();
            sys.refresh_memory();

            // CPU usage: sysinfo gives percent (0..100) for global CPU.
            let cpu = sys.global_cpu_info().cpu_usage() as f64;
            HOST_CPU_USAGE.set(cpu);

            // Memory: sysinfo returns KiB in many versions; convert to bytes.
            let total_bytes = (sys.total_memory() as f64) * 1024.0;
            let used_bytes = (sys.used_memory() as f64) * 1024.0;

            HOST_MEM_TOTAL_BYTES.set(total_bytes);
            HOST_MEM_USED_BYTES.set(used_bytes);
        }
    });
}

pub async fn metrics_handler() -> Result<Response<Body>, (StatusCode, String)> {
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();

    let mut buf = Vec::new();
    encoder
        .encode(&metric_families, &mut buf)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let mut resp = Response::new(Body::from(buf));

    // Prometheus expects this content type for the text exposition format
    let content_type = encoder.format_type(); // e.g. "text/plain; version=0.0.4"
    resp.headers_mut().insert(
        header::CONTENT_TYPE,
        HeaderValue::from_str(content_type)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?,
    );

    Ok(resp)
}

pub async fn run_metrics_server(bind_addr: &str) {
    let app = Router::new().route("/metrics", get(metrics_handler));

    let listener = tokio::net::TcpListener::bind(bind_addr)
        .await
        .unwrap_or_else(|e| panic!("failed to bind metrics server to {bind_addr}: {e}"));

    axum::serve(listener, app).await.expect("serve");
}
