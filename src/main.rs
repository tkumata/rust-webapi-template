use axum::{
    routing::{get, post},
    Router,
};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
    .route(
        "/",
        get(|| async {
            "Hello, Rust World!"
        })
    )
    .route(
        "/foo",
        get(|| async {
            "Hello, foo World."
        })
    )
    .route(
        "/bar",
        get(|| async {
            "Hello, bar world."
        })
    );

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
