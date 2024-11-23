use dotenvx::dotenv;
use sqlx::{Pool, Postgres};
use std::env;

pub async fn create_db_pool() -> Result<Pool<Postgres>, sqlx::Error> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    sqlx::PgPool::connect(&database_url).await
}
