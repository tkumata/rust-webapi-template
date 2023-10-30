use axum::{http::StatusCode, response::IntoResponse, response::Json};
use serde::Serialize;
use serde_json::json;
use tokio::time::{sleep, Duration};

#[derive(Serialize)]
struct SleepDuration {
    sleep_duration: i32
}

pub async fn make_sleep() -> impl IntoResponse {
    sleep(Duration::from_secs(30)).await;

    let duration = SleepDuration { sleep_duration: 30 };

    (StatusCode::OK, Json(json!(duration)))
}
