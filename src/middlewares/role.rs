use crate::error::AppError;
use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
};

pub async fn admin_guard(
    request: Request,
    next: Next,
) -> Result<Response, AppError> {
    Ok(next.run(request).await)
}
