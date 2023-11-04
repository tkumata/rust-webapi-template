use axum::{http::StatusCode, response::IntoResponse, response::Json};
use serde::Serialize;
use serde_json::json;

use crate::models;

#[derive(Serialize)]
struct Metrics {
    kernel_name: Option<String>,
    cpu_load: String,
    memory_usage: u64,
    disk_info: String,
}

pub async fn get_metrics() -> impl IntoResponse {
    let kernel_name = models::metrics::get_kernelname();
    let cpu_load = models::metrics::get_load();
    let mem_usage = models::metrics::get_mem();
    let disk_info = models::metrics::get_storage();

    let metrics = Metrics {
        kernel_name: kernel_name.await,
        cpu_load: cpu_load.await,
        memory_usage: mem_usage.await,
        disk_info: disk_info.await,
    };

    (StatusCode::OK, Json(json!(metrics)))
}
