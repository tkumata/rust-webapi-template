use crate::domain::user::User;
use bcrypt::verify;
use sqlx::PgPool;

pub async fn authenticate_user(
    pool: &PgPool,
    email: &str,
    password: &str,
) -> Result<User, &'static str> {
    let user = sqlx::query_as!(
        User,
        "SELECT id, name, email, password_hash FROM users WHERE email = $1",
        email
    )
    .fetch_one(pool)
    .await
    .map_err(|_| "Invalid email or password")?;

    // パスワードを検証
    let is_valid =
        verify(password, &user.password_hash).map_err(|_| "Failed to verify password")?;

    if is_valid {
        Ok(user)
    } else {
        Err("Invalid email or password")
    }
}
