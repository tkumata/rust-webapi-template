use axum::{
    http::StatusCode,
    response::IntoResponse,
    response::Json
};
use serde::{
    Serialize,
    Deserialize
};
use serde_json::json;

#[derive(Deserialize)]
pub struct RequestRgb {
    red: i32,
    green: i32,
    blue: i32,
}

#[derive(Serialize)]
struct ResponseRgb {
    red: String,
    green: String,
    blue: String
}

// Todo: sparate logic
// Todo: validate input data format

pub async fn make_rgb(Json(req_rgb): Json<RequestRgb>) -> impl IntoResponse {
    let converted = rgb(req_rgb.red, req_rgb.green, req_rgb.blue);
    let rgb_hex = ResponseRgb {
        red: converted[0].to_owned(),
        green: converted[1].to_owned(),
        blue: converted[2].to_owned(),
    };

    (StatusCode::OK, Json(json!(rgb_hex)))
}

pub fn to_hex(value: i32) -> String {
    let value = if value > 255 { 255 } else if value < 0 { 0 } else { value };
    format!("{:02X}", value)
}

pub fn rgb(r: i32, g: i32, b: i32) -> Vec<String> {
    let mut rgb = Vec::new();
    rgb.push(to_hex(r).to_string());
    rgb.push(to_hex(g).to_string());
    rgb.push(to_hex(b).to_string());

    rgb
}
