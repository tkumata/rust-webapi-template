use axum::{http::StatusCode, response::IntoResponse, response::Json};
use serde_json::json;

pub async fn root() -> impl IntoResponse {
    (StatusCode::OK, Json(json!("OK")))
}
