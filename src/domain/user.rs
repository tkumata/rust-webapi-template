use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    #[serde(skip_serializing)] // パスワードハッシュは JSON に含めない
    pub password_hash: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
    #[serde(skip_serializing)] // パスワードハッシュは JSON に含めない
    pub password: String,
}

#[derive(Deserialize)]
pub struct UpdateUser {
    pub name: Option<String>,
    pub email: Option<String>
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}
