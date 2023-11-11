use axum::{
    http::StatusCode,
    response::IntoResponse,
    response::Json
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RequestRgb {
    r: i32,
    g: i32,
    b: i32,
}

struct ConvertedRgb {
    r: String,
    g: String,
    b: String
}

// Todo: sparate logic

pub async fn rgb(Json(req_rgb): Json<RequestRgb>) -> impl IntoResponse {
    let converted = convert_rgb(req_rgb.r, req_rgb.g, req_rgb.b);
    let rgb = format!("#{}{}{}", converted.r, converted.g, converted.b);
    (StatusCode::OK, rgb)
}

fn to_hex(value: i32) -> String {
    let value = if value > 255 { 255 } else if value < 0 { 0 } else { value };
    format!("{:02X}", value)
}

fn convert_rgb(r: i32, g: i32, b: i32) -> ConvertedRgb {
    let rgb = ConvertedRgb {
        r: to_hex(r).to_string(),
        g: to_hex(g).to_string(),
        b: to_hex(b).to_string()
    };
    rgb
}
