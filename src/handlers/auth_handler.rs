use axum::{
    extract::State,
    Json,
};
use crate::{
    dtos::user::{CreateUser, UserResponse},
    error::AppError,
    state::AppState,
};
use chrono::{Utc, Duration};
use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserResponse,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    role: String,
    exp: usize,
}

pub struct AuthHandler;

impl AuthHandler {

    pub async fn register(
        State(state): State<AppState>,
        Json(payload): Json<CreateUser>,
    ) -> Result<Json<UserResponse>, AppError> {
        payload.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;

        let user = state.user_service.create_user(payload).await?;
        Ok(Json(user))
    }

    pub async fn login(
        State(state): State<AppState>,
        Json(payload): Json<LoginRequest>,
    ) -> Result<Json<AuthResponse>, AppError> {
        payload.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;

        let user = state.user_service.authenticate(&payload.email, &payload.password).await?;

        // Generate JWT
        let exp = Utc::now()
            .checked_add_signed(Duration::hours(24))
            .expect("valid timestamp")
            .timestamp() as usize;

        let claims = Claims {
            sub: user.id.clone().unwrap(),
            role: user.role.clone(),
            exp,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(state.config.jwt_secret.as_ref()),
        ).map_err(|_| AppError::AuthError)?;

        Ok(Json(AuthResponse {
            token,
            user: user.into(),
        }))
    }
}
