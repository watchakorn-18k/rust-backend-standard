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
    #[error("User already exists")]
    UserAlreadyExists,
    #[error("Invalid email or password")]
    InvalidCredentials,
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
            AppError::UserAlreadyExists => (StatusCode::CONFLICT, "User already exists"),
            AppError::InvalidCredentials => (StatusCode::UNAUTHORIZED, "Invalid email or password"),
        };

        let body = Json(json!({
            "ok": false,
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::response::IntoResponse;

    #[tokio::test]
    async fn test_error_into_response() {
        assert_eq!(AppError::NotFound.into_response().status(), StatusCode::NOT_FOUND);
        assert_eq!(AppError::AuthError.into_response().status(), StatusCode::UNAUTHORIZED);
        
        let res = AppError::ValidationError("test".into()).into_response();
        assert_eq!(res.status(), StatusCode::BAD_REQUEST);

        let res = AppError::DatabaseError(mongodb::error::Error::custom("db error")).into_response();
        assert_eq!(res.status(), StatusCode::INTERNAL_SERVER_ERROR);

        let res = AppError::InternalServerError.into_response();
        assert_eq!(res.status(), StatusCode::INTERNAL_SERVER_ERROR);

        let res = AppError::PermissionDenied.into_response();
        assert_eq!(res.status(), StatusCode::FORBIDDEN);

        let res = AppError::InvalidCredentials.into_response();
        assert_eq!(res.status(), StatusCode::UNAUTHORIZED);

        assert_eq!(AppError::UserAlreadyExists.into_response().status(), StatusCode::CONFLICT);
        
        let res = AppError::AnyError(anyhow::anyhow!("error")).into_response();
        assert_eq!(res.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }
}
