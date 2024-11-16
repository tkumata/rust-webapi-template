mod controllers;
mod models;
mod utils;
// mod repositories;
mod domain;
mod infrastructure;
mod application;
mod presentation;

use axum::{Router, routing::get, routing::post, routing::put, routing::delete};
use infrastructure::db::create_db_pool;
use presentation::user_handler;

#[tokio::main(flavor = "multi_thread", worker_threads = 16)]
async fn main() {
    let pool = create_db_pool().await.expect("Failed to create DB pool");

    let app = Router::new()
        .route("/", get(controllers::root::root))
        // .route("/todo/add", post(controllers::todo::add))
        .route("/dice", get(controllers::dice::roll))
        .route("/sleep/:wait_time", get(controllers::sleepy::put_to_sleep))
        .route("/metrics", get(controllers::metrics::get_metrics))
        .route("/convert/v4prefix", post(controllers::convert::v4_prefix))
        .route("/convert/rgb", post(controllers::convert::rgb))
        .route("/users", get(user_handler::list_users))
        .route("/users", post(user_handler::create_user))
        .route("/users/:id", get(user_handler::get_user))
        .route("/users/:id", put(user_handler::update_user))
        .route("/users/:id", delete(user_handler::delete_user))
        .with_state(pool);

        axum::Server::bind(&"0.0.0.0:4000".parse().unwrap())
            .serve(app.into_make_service())
            .await
            .unwrap();
}
