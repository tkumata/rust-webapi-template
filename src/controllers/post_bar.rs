use axum::{http::StatusCode, response::IntoResponse, response::Json};
use serde::Serialize;
use serde_json::json;

#[derive(Serialize)]
struct PostBar {
    bar: String
}

pub async fn post_bar() -> impl IntoResponse {
    let message = "Hello, POST /bar Rust World with handler and controllers.".to_string();
    let bar = PostBar { bar : message };

    (StatusCode::OK, Json(json!(bar)))
}
