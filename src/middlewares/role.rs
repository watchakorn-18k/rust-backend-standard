use crate::error::AppError;
use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
};

#[allow(dead_code)]
pub async fn admin_guard(
    request: Request,
    next: Next,
) -> Result<Response, AppError> {
    Ok(next.run(request).await)
}
