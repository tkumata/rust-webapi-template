use crate::domain::user::{CreateUser, UpdateUser, User};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub async fn get_all_users(pool: &Pool<Postgres>) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as!(User, "SELECT id, name, email FROM users")
        .fetch_all(pool)
        .await
}

pub async fn get_user_by_id(pool: &Pool<Postgres>, id: Uuid) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as!(User, "SELECT id, name, email FROM users WHERE id = $1", id)
        .fetch_optional(pool)
        .await
}

pub async fn create_user(pool: &Pool<Postgres>, input: CreateUser) -> Result<User, sqlx::Error> {
    sqlx::query_as!(
        User,
        "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id, name, email",
        input.name,
        input.email
    )
    .fetch_one(pool)
    .await
}

pub async fn update_user(
    pool: &Pool<Postgres>,
    id: Uuid,
    input: UpdateUser,
) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as!(
        User,
        "UPDATE users SET name = COALESCE($1, name), email = COALESCE($2, email) WHERE id = $3 RETURNING id, name, email",
        input.name,
        input.email,
        id
    )
    .fetch_optional(pool)
    .await
}

pub async fn delete_user(pool: &Pool<Postgres>, id: Uuid) -> Result<u64, sqlx::Error> {
    let result = sqlx::query!("DELETE FROM users WHERE id = $1", id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected())
}
