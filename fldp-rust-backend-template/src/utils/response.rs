use axum::{Json, response::IntoResponse, http::StatusCode};
use serde::Serialize;
use serde_json::json;

pub fn json_ok<T: Serialize>(data: T) -> impl IntoResponse {
    (StatusCode::OK, Json(json!({ "ok": true, "data": data })))
}

pub fn json_created<T: Serialize>(data: T) -> impl IntoResponse {
    (StatusCode::CREATED, Json(json!({ "ok": true, "data": data })))
}
