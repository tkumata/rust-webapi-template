use axum::{
    http::StatusCode,
    response::IntoResponse,
    response::Json,
    routing::{get, post},
    Router,
};
use serde_json::json;

mod controllers;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/dice", get(controllers::dice::get_dice))
        .route("/bar", post(post_bar));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> impl IntoResponse {
    (StatusCode::OK, Json(json!("OK")))
}

async fn post_bar() -> &'static str {
    "Hello, Rust POST /bar World! and handler function."
}
