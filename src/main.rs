use axum::{
    routing::{get, post},
    Router,
};
// use serde_json::json;

mod controllers;
mod models;

#[tokio::main(flavor = "multi_thread", worker_threads = 16)]
async fn main() {
    let app = Router::new()
        .route(
            "/",
            get(controllers::root::root)
        )
        .route(
            "/dice",
            get(controllers::dice::roll)
        )
        .route(
            "/sleep/:wait_time",
            get(controllers::sleepy::put_to_sleep)
        )
        .route(
            "/metrics",
            get(controllers::metrics::get_metrics)
        )
        .route(
            "/bar",
            post(controllers::bar::bar)
        );

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
