use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use headers::{Authorization, HeaderMapExt};
use headers::authorization::Bearer;
use crate::auth::jwt::validate_token;
use sqlx::PgPool;

pub struct AuthenticatedUser(pub String);

#[async_trait]
impl FromRequestParts<PgPool> for AuthenticatedUser {
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(
        parts: &mut Parts,
        pool: &PgPool,
    ) -> Result<Self, Self::Rejection> {
        // Authorization ヘッダーを取得
        let auth_header = parts.headers.typed_get::<Authorization<Bearer>>();
        let token = if let Some(Authorization(bearer)) = auth_header {
            bearer.token().to_string() // トークンをクローン
        } else {
            return Err((StatusCode::UNAUTHORIZED, "Missing token"));
        };

        // トークンの検証
        let claims = validate_token(&token).map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid token"))?;

        // データベースでトークンを確認
        let token_exists = sqlx::query!(
            "SELECT COUNT(*) FROM user_tokens WHERE token = $1",
            token
        )
        .fetch_one(pool)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Database error"))?
        .count
        .unwrap_or(0);

        if token_exists == 0 {
            return Err((StatusCode::UNAUTHORIZED, "Token not found"));
        }

        Ok(AuthenticatedUser(claims.sub))
    }
}
