use axum::{
    extract::State,
    Json,
};
use crate::{
    dtos::user::{CreateUser, UserResponse},
    error::AppError,
    models::user::User,
    state::AppState,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Utc, Duration};
use jsonwebtoken::{encode, Header, EncodingKey};
use mongodb::bson::doc;
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

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<UserResponse>, AppError> {
    payload.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;

    // Check if user exists
    let existing_user = state.db.collection::<User>("users")
        .find_one(doc! { "email": &payload.email }, None)
        .await?;

    if existing_user.is_some() {
        return Err(AppError::UserAlreadyExists);
    }

    let hashed_password = hash(payload.password, DEFAULT_COST)
        .map_err(|_| AppError::AuthError)?;

    let user_id = uuid::Uuid::new_v4().to_string();

    let new_user = User {
        id: Some(user_id),
        username: payload.username,
        email: payload.email,
        password_hash: hashed_password,
        role: "user".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    state.db.collection::<User>("users")
        .insert_one(new_user.clone(), None)
        .await?;

    let user_response: UserResponse = new_user.into();

    Ok(Json(user_response))
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    payload.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;

    let user = state.db.collection::<User>("users")
        .find_one(doc! { "email": &payload.email }, None)
        .await?
        .ok_or(AppError::InvalidCredentials)?;

    if !verify(payload.password, &user.password_hash).map_err(|_| AppError::InvalidCredentials)? {
        return Err(AppError::InvalidCredentials);
    }

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
