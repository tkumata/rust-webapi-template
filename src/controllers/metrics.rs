use axum::{http::StatusCode, response::IntoResponse, response::Json};
use serde::Serialize;
use serde_json::json;
use sysinfo::{System, SystemExt};

#[derive(Serialize)]
struct Metrics {
    mem: u64,
}

pub async fn metrics() -> impl IntoResponse {
    let sys = System::new_all();
    let metrics = Metrics {
        mem: sys.used_memory(),
    };

    (StatusCode::OK, Json(json!(metrics)))
}
