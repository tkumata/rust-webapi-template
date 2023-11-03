use axum::{
    // http::StatusCode,
    // response::IntoResponse,
    // response::Json,
    routing::{get, post},
    Router,
};
// use serde_json::json;

mod controllers;
mod models;

#[tokio::main(flavor = "multi_thread", worker_threads = 16)]
async fn main() {
    let app = Router::new()
        .route("/", get(controllers::root::root))
        .route("/dice", get(controllers::dice::dice))
        .route("/sleep/:wait_time", get(controllers::make_sleep::make_sleep))
        .route("/metrics", get(controllers::get_metrics::get_metrics))
        .route("/bar", post(controllers::bar::bar));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
