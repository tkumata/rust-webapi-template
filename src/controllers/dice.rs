use axum::{http::StatusCode, response::IntoResponse, response::Json};
use rand::Rng;
use serde::Serialize;
use serde_json::json;

#[derive(Serialize)]
struct Dice {
    dice: i32,
}

pub async fn get_dice() -> impl IntoResponse {
    let mut rnd: rand::rngs::ThreadRng = rand::thread_rng();
    let i: i32 = rnd.gen_range(1..6);
    let dice = Dice { dice: i };

    (StatusCode::OK, Json(json!(dice)))
}
