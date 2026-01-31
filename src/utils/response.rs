use axum::{Json, response::IntoResponse, http::StatusCode};
use serde::Serialize;
use serde_json::json;

pub fn json_ok<T: Serialize>(data: T) -> impl IntoResponse {
    (StatusCode::OK, Json(json!({ "ok": true, "data": data })))
}

pub fn json_created<T: Serialize>(data: T) -> impl IntoResponse {
    (StatusCode::CREATED, Json(json!({ "ok": true, "data": data })))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::response::IntoResponse;

    #[tokio::test]
    async fn test_json_ok() {
        let res = json_ok("test").into_response();
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_json_created() {
        let res = json_created(123).into_response();
        assert_eq!(res.status(), StatusCode::CREATED);
    }
}
