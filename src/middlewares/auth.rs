use crate::error::AppError;
use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use crate::state::AppState;

pub async fn auth_middleware(
    State(_state): State<AppState>, // Can be used to check blocklist etc.
    request: Request,
    next: Next,
) -> Result<Response, AppError> {
    // 1. Get Authorization header
    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok())
        .ok_or(AppError::AuthError)?;

    // 2. Validate JWT (Placeholder logic)
    if !auth_header.starts_with("Bearer ") {
        return Err(AppError::AuthError);
    }
    
    let _token = &auth_header[7..];
    // TODO: Validate token using jsonwebtoken crate
    // let token_data = decode::<Claims>(token, &DecodingKey::from_secret(secret), &Validation::default())?;

    // 3. Insert user info into request extensions
    // request.extensions_mut().insert(token_data.claims);
    
    Ok(next.run(request).await)
}
