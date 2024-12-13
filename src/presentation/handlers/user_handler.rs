use crate::{
    application::user_service,
    auth::middleware::AuthenticatedUser,
    domain::user::{CreateUser, UpdateUser},
};
use axum::{extract::State, Json};
use sqlx::PgPool;
use uuid::Uuid;

// 全ユーザ情報取得ハンドラ
// pub async fn list_users(
//     State(pool): State<PgPool>,
//     AuthenticatedUser(_claims): AuthenticatedUser, // 認証を要求
// ) -> Json<Vec<crate::domain::user::User>> {
//     let users = user_service::get_all_users(&pool).await.unwrap();
//     Json(users)
// }

// 個別ユーザ情報取得ハンドラ
pub async fn get_user(
    State(pool): State<PgPool>,
    AuthenticatedUser(claims): AuthenticatedUser, // 認証を要求
) -> Json<Option<crate::domain::user::User>> {
    // 認証が成功した user id を Uuid 型 id に束縛する。
    let id = Uuid::parse_str(&claims).expect("Invalid UUID.");

    // 借用した DB 接続オブジェクトと上記 id で個のユーザ情報を取得する。
    let user = user_service::get_user_by_id(&pool, id).await.unwrap();
    Json(user)
}

// ユーザ作成ハンドラ
// ユーザ作成は認証なし
pub async fn create_user(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateUser>,
) -> Json<crate::domain::user::User> {
    let user = user_service::create_user(&pool, payload).await.unwrap();
    Json(user)
}

// ユーザ情報更新ハンドラ
pub async fn update_user(
    State(pool): State<PgPool>,
    AuthenticatedUser(claims): AuthenticatedUser, // 認証を要求
    Json(payload): Json<UpdateUser>,
) -> Json<Option<crate::domain::user::User>> {
    // 認証が成功した user id を Uuid 型 id に束縛する。
    let id = Uuid::parse_str(&claims).expect("Invalid UUID.");

    // 借用した DB 接続オブジェクトと上記 id と送信された payload で個のユーザ情報を更新する。
    let user = user_service::update_user(&pool, id, payload).await.unwrap();
    Json(user)
}

// ユーザ情報削除ハンドラ
pub async fn delete_user(
    State(pool): State<PgPool>,
    AuthenticatedUser(claims): AuthenticatedUser,
) -> Json<u64> {
    // 認証が成功した user id を Uuid 型 id に束縛する。
    let id = Uuid::parse_str(&claims).expect("Invalid UUID.");

    // 借用した DB 接続オブジェクトと上記 id で個のユーザ情報を削除する。
    let rows_affected = user_service::delete_user(&pool, id).await.unwrap();
    Json(rows_affected)
}
