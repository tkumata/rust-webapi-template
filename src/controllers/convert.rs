use axum::{
    http::StatusCode,
    response::IntoResponse,
    response::Json
};
use serde::Deserialize;

use crate::utils;

#[derive(Deserialize)]
pub struct RequestRgb {
    r: i32,
    g: i32,
    b: i32
}

pub async fn rgb(Json(req_rgb): Json<RequestRgb>) -> impl IntoResponse {
    let converted = utils::utils::convert_rgb(req_rgb.r, req_rgb.g, req_rgb.b);
    let rgb = format!("#{}{}{}", converted.r, converted.g, converted.b);
    (StatusCode::OK, rgb)
}
