use crate::{error::AppError, state::AppState};
use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};

pub async fn auth_middleware(
    State(_state): State<AppState>,
    request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok())
        .ok_or(AppError::AuthError)?;

    if !auth_header.starts_with("Bearer ") {
        return Err(AppError::AuthError);
    }
    
    Ok(next.run(request).await)
}
