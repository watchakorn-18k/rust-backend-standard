use crate::error::AppError;
use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
};

// Example role checker
pub async fn admin_guard(
    request: Request,
    next: Next,
) -> Result<Response, AppError> {
    // 1. Get user from extension (inserted by auth middleware)
    // let user = request.extensions().get::<Claims>().ok_or(AppError::AuthError)?;
    
    // 2. Check role
    // if user.role != "admin" { return Err(AppError::PermissionDenied); }

    Ok(next.run(request).await)
}
