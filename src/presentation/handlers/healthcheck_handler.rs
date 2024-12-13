use axum::{http::StatusCode, response::IntoResponse, response::Json};
use serde_json::json;

pub async fn healthcheck() -> impl IntoResponse {
    (StatusCode::OK, Json(json!("OK")))
}
