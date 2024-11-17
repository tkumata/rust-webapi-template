use bcrypt::{hash, DEFAULT_COST};
use crate::domain::user::{CreateUser, UpdateUser, User};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub async fn get_all_users(
    pool: &Pool<Postgres>
) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as!(User, "SELECT id, name, email, password_hash FROM users")
        .fetch_all(pool)
        .await
}

pub async fn get_user_by_id(
    pool: &Pool<Postgres>,
    id: Uuid
) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as!(User, "SELECT id, name, email, password_hash FROM users WHERE id = $1", id)
        .fetch_optional(pool)
        .await
}

pub async fn create_user(
    pool: &Pool<Postgres>,
    input: CreateUser
) -> Result<User, sqlx::Error> {
    // パスワードをハッシュ化
    let password_hash = hash(input.password, DEFAULT_COST)
        .map_err(|_| sqlx::Error::RowNotFound)?; // bcrypt エラーを適切に処理

    // uuid を生成
    let user_id = Uuid::new_v4();

    // ユーザ情報をデータベースに保存
    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (id, name, email, password_hash)
        VALUES ($1, $2, $3, $4)
        RETURNING id, name, email, password_hash
        "#,
        user_id,
        input.name,
        input.email,
        password_hash
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
}

pub async fn update_user(
    pool: &Pool<Postgres>,
    id: Uuid,
    input: UpdateUser,
) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"
        UPDATE users SET name = COALESCE($1, name), email = COALESCE($2, email)
        WHERE id = $3
        RETURNING id, name, email, password_hash
        "#,
        input.name,
        input.email,
        id
    )
    .fetch_optional(pool)
    .await
}

pub async fn delete_user(
    pool: &Pool<Postgres>,
    id: Uuid
) -> Result<u64, sqlx::Error> {
    let result = sqlx::query!("DELETE FROM users WHERE id = $1", id)
        .execute(pool)
        .await?;

    Ok(result.rows_affected())
}
