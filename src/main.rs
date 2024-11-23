mod application;
mod auth;
mod domain;
mod infrastructure;
mod presentation;

use axum::{
    routing::{delete, get, post, put},
    Router,
};
use infrastructure::db::create_db_pool;
use presentation::auth_handler;
use presentation::healthcheck_handler;
use presentation::user_handler;

#[tokio::main(flavor = "multi_thread", worker_threads = 16)]
async fn main() {
    let pool = create_db_pool().await.expect("Failed to create DB pool");

    let app = Router::new()
        // healthcheck
        .route("/healthcheck", get(healthcheck_handler::healthcheck))
        // Auth and make one hour token.
        .route("/login", post(auth_handler::login))
        // Get all users.
        // .route("/users", get(user_handler::list_users))
        // Create user.
        .route("/user", post(user_handler::create_user))
        // Get a user.
        .route("/user", get(user_handler::get_user))
        // Update a user.
        .route("/user", put(user_handler::update_user))
        // Delete a user.
        .route("/user", delete(user_handler::delete_user))
        .with_state(pool);

    // run our app with hyper, listening globally on port 4000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
