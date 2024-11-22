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
    // email と password で DB を参照し登録ユーザか確認する。
    let user = authenticate_user(&pool, &payload.email, &payload.password)
        .await
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid email or password"))?;

    // 上記 map_err() のおかげで正常なユーザは処理が続行される。
    // JWT トークンを生成する。
    let token = create_and_store_token(user.id, &pool)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to generate token"))?;

    Ok(Json(token))
}
