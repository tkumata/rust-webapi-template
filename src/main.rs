use axum::{
    routing::{get, post},
    Router,
};

#[tokio::main]
async fn main() {
    // build our application with a single route
    // Todo: プラグイン的にとっかえひっかえができるようなカタチにする。
    let app = Router::new()
        .route(
            "/",
            get(root)
        )
        .route(
            "/foo",
            get(get_foo)
        )
        .route(
            "/bar",
            post(post_bar)
        );

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, Rust World!"
}

async fn get_foo() -> &'static str {
    "Hello, Rust GET /foo World! and handler function."
}

async fn post_bar() -> &'static str {
    "Hello, Rust POST /bar World! and handler function."
}
