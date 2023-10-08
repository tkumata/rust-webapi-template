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
            get(|| async {
                "Hello, Rust World!"
            })
        )
        .route(
            "/foo",
            get(|| async {
                "Hello, Rust GET /foo World!"
            })
        )
        .route(
            "/post",
            post(|| async {
                "Hello, Rust POST /post World."
            })
        );

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
