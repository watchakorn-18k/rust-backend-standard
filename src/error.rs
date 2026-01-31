use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Not Found")]
    NotFound,
    #[error("Invalid Input: {0}")]
    ValidationError(String),
    #[error("Authentication Failed")]
    AuthError,
    #[allow(dead_code)]
    #[error("Permission Denied")]
    PermissionDenied,
    #[error("Database Error: {0}")]
    DatabaseError(#[from] mongodb::error::Error),
    #[error("Internal Server Error")]
    InternalServerError,
    #[error("Any Error: {0}")]
    AnyError(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::NotFound => (StatusCode::NOT_FOUND, "Resource not found"),
            AppError::ValidationError(msg) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({ "ok": false, "error": "Validation Error", "details": msg })),
                )
                    .into_response();
            }
            AppError::AuthError => (StatusCode::UNAUTHORIZED, "Authentication failed"),
            AppError::PermissionDenied => (StatusCode::FORBIDDEN, "Permission denied"),
            AppError::DatabaseError(e) => {
                tracing::error!("Database Error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error")
            }
            AppError::InternalServerError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            AppError::AnyError(e) => {
                tracing::error!("Unexpected Error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
        };

        let body = Json(json!({
            "ok": false,
            "error": error_message,
        }));

        (status, body).into_response()
    }
}
