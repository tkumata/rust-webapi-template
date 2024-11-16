use axum::{
    http::StatusCode,
    response::IntoResponse,
    response::Json,
    extract::Path
};
use serde::{Serialize, Deserialize};
use serde_json::json;
use tokio::time::{sleep, Duration};

#[derive(Deserialize, Serialize)]
pub struct WaitDurationPath {
    wait_time: i32,
}

#[derive(Serialize)]
struct SleepDuration {
    sleep_duration: i32
}

pub async fn put_to_sleep(
    Path(path): Path<WaitDurationPath>
) -> impl IntoResponse {

    let d = path.wait_time as u64;
    sleep(Duration::from_secs(d)).await;  // threading 中なので tokio の sleep を利用する。

    let duration = SleepDuration { sleep_duration: path.wait_time };

    (StatusCode::OK, Json(json!(duration)))
}
