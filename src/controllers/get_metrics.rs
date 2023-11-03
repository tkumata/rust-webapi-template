use axum::{http::StatusCode, response::IntoResponse, response::Json};
use serde::Serialize;
use serde_json::json;

use crate::models;

#[derive(Serialize)]
struct Metrics {
    name: Option<String>,
    cpu_load: String,
    memory_usage: u64,
}

pub async fn get_metrics() -> impl IntoResponse {
    let os_name = models::metrics::get_osname();
    let cpu_load = models::metrics::get_load();
    let mem_usage = models::metrics::get_mem();

    let metrics = Metrics {
        name: os_name.await,
        cpu_load: cpu_load.await,
        memory_usage: mem_usage.await,
    };

    (StatusCode::OK, Json(json!(metrics)))
}
