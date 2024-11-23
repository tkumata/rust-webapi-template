use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use std::env;
use uuid::Uuid;

// トークンに埋め込むクレーム
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // Subject (ユーザー ID)
    pub exp: usize,  // Expiration time (UNIX timestamp)
}

/// JWT トークンを生成してデータベースに保存
pub async fn create_and_store_token(
    user_id: Uuid,
    pool: &Pool<Postgres>,
) -> Result<String, sqlx::Error> {
    let jwt_secret: String = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let secret_key: &[u8] = jwt_secret.as_bytes();

    // トークンの有効期限を設定
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(2))
        .expect("Failed to calculate expiration")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration,
    };

    // JWT トークンを生成
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret_key),
    )
    .expect("Failed to encode JWT");

    // トークンをデータベースに保存
    let result = sqlx::query!(
        "INSERT INTO user_tokens (id, user_id, token, expires_at) VALUES ($1, $2, $3, TO_TIMESTAMP($4))",
        Uuid::new_v4(),
        user_id,
        token,
        expiration as i64,
    )
    .execute(pool)
    .await;

    if let Err(err) = result {
        println!("SQL execution error: {:?}", err);
    }

    Ok(token)
}
