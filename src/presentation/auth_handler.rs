use axum::{extract::State, Json};
use crate::domain::user::LoginRequest;
use crate::auth::auth_service::authenticate_user;
use crate::application::token_service::create_and_store_token;
use sqlx::PgPool;
use axum::http::StatusCode;

pub async fn login(
    State(pool): State<PgPool>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<String>, (StatusCode, &'static str)> {
    let user = authenticate_user(&pool, &payload.email, &payload.password)
        .await
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid email or password"))?;

    let token = create_and_store_token(user.id, &pool)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to generate token"))?;

    Ok(Json(token))
}
