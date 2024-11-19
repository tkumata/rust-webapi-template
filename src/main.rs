mod models;
mod auth;
mod domain;
mod infrastructure;
mod application;
mod presentation;

use axum::{Router, routing::{get, post, put, delete}};
use infrastructure::db::create_db_pool;
use presentation::user_handler;
use presentation::auth_handler;
use presentation::convert_handler;
use presentation::metrics_handler;
use presentation::healthcheck_handler;
use presentation::dice_handler;
use presentation::sleep_handler;

#[tokio::main(flavor = "multi_thread", worker_threads = 16)]
async fn main() {
    let pool = create_db_pool().await.expect("Failed to create DB pool");

    let app = Router::new()
        // healthcheck
        .route("/healthcheck", get(healthcheck_handler::healthcheck))
        // Return random number
        .route("/roll/1d6", get(dice_handler::roll_1d6))
        // Sleep
        .route("/sleep/:wait_time", get(sleep_handler::make_sleep))
        // Get metrics.
        .route("/metrics", get(metrics_handler::get_metrics))
        // Convert /27 to 255.255.255.224
        .route("/convert/v4prefix", post(convert_handler::convert_v4prefix))
        // Convert 55,155,250 to 379BFA
        .route("/convert/rgb", post(convert_handler::convert_rgb))
        // Auth and make one hour token.
        .route("/login", post(auth_handler::login))
        // Get all users.
        .route("/users", get(user_handler::list_users))
        // Create user.
        .route("/users", post(user_handler::create_user))
        // Get a user.
        .route("/users/:id", get(user_handler::get_user))
        // Update a user.
        .route("/users/:id", put(user_handler::update_user))
        // Delete a user.
        .route("/users/:id", delete(user_handler::delete_user))
        .with_state(pool);

        // run our app with hyper, listening globally on port 4000
        let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();
        axum::serve(listener, app).await.unwrap();
}
