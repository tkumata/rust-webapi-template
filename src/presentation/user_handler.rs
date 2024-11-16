use axum::{extract::{Path, State}, Json};
use crate::{
    application::user_service,
    domain::user::{CreateUser, UpdateUser},
};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn list_users(State(pool): State<PgPool>) -> Json<Vec<crate::domain::user::User>> {
    let users = user_service::get_all_users(&pool).await.unwrap();
    Json(users)
}

pub async fn get_user(State(pool): State<PgPool>, Path(id): Path<Uuid>) -> Json<Option<crate::domain::user::User>> {
    let user = user_service::get_user_by_id(&pool, id).await.unwrap();
    Json(user)
}

pub async fn create_user(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateUser>,
) -> Json<crate::domain::user::User> {
    let user = user_service::create_user(&pool, payload).await.unwrap();
    Json(user)
}

pub async fn update_user(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateUser>,
) -> Json<Option<crate::domain::user::User>> {
    let user = user_service::update_user(&pool, id, payload).await.unwrap();
    Json(user)
}

pub async fn delete_user(State(pool): State<PgPool>, Path(id): Path<Uuid>) -> Json<u64> {
    let rows_affected = user_service::delete_user(&pool, id).await.unwrap();
    Json(rows_affected)
}
