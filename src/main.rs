use axum::{
    routing::{delete, get, post, put},
    Router,
};
use clap::Command;
use dotenvx::dotenv;
use std::env;
use tokio::net::TcpListener;
use tracing::info;

mod application;
mod domain;
mod infrastructure;
mod presentation;

use infrastructure::db::create_db_pool;
use presentation::handlers::auth_handler;
use presentation::handlers::healthcheck_handler;
use presentation::handlers::user_handler;

#[tokio::main(flavor = "multi_thread", worker_threads = 16)]
async fn main() -> anyhow::Result<()> {
    // 環境変数の読み込み
    dotenv().ok();

    // ログの初期化
    tracing_subscriber::fmt::init();

    info!("🚦 Starting application...");

    // コマンドライン引数の処理
    parse_command_line();

    // データベース接続プール作成
    let pool = create_db_pool().await?;

    // ルータ作成
    let app_router = build_app_router(pool);

    // アプリ起動
    run_server(app_router).await?;

    Ok(())
}

fn parse_command_line() {
    let _matches = Command::new("rust-webapi-template")
        .version("0.0.1")
        .author("Tomokatsu Kumata")
        .about("REST API in Rust⚙")
        .get_matches();
}

fn build_app_router(pool: sqlx::Pool<sqlx::Postgres>) -> Router {
    let healthcheck_routes =
        Router::new().route("/healthcheck", get(healthcheck_handler::healthcheck));

    let auth_routes = Router::new().route("/login", post(auth_handler::login));

    let user_routes = Router::new()
        .route("/user", post(user_handler::create_user))
        .route("/user", get(user_handler::get_user))
        .route("/user", put(user_handler::update_user))
        .route("/user", delete(user_handler::delete_user));

    Router::new()
        .merge(healthcheck_routes)
        .merge(auth_routes)
        .merge(user_routes)
        .with_state(pool)
}

async fn run_server(app_router: Router) -> anyhow::Result<()> {
    let address = env::var("APP_ADDRESS").unwrap_or_else(|_| "0.0.0.0:4000".to_string());
    let listener = TcpListener::bind(&address).await?;
    info!("🚀 Listening on {}", address);
    axum::serve(listener, app_router).await?;
    Ok(())
}
