use crate::{
    dtos::user::{CreateUser, UpdateUser},
    error::AppError,
    repositories::user_repository::UserRepository,
    services::user_service::UserService,
    state::AppState,
    utils::response::{json_created, json_ok,},
    utils::pagination::PaginationParams,
};
use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    Json,
};
use mongodb::bson::oid::ObjectId;
use validator::Validate;

// Helper to get service
fn get_service(state: &AppState) -> UserService {
    UserService::new(UserRepository::new(&state.db))
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> Result<impl IntoResponse, AppError> {
    payload.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;
    
    let service = get_service(&state);
    let user = service.create_user(payload).await?;
    
    Ok(json_created(user))
}

pub async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let oid = ObjectId::parse_str(&id).map_err(|_| AppError::ValidationError("Invalid ID format".into()))?;
    
    let service = get_service(&state);
    let user = service.get_user(oid).await?;
    
    Ok(json_ok(user))
}

pub async fn list_users(
    State(state): State<AppState>,
    Query(params): Query<PaginationParams>,
) -> Result<impl IntoResponse, AppError> {
    let service = get_service(&state);
    let result = service.list_users(params.page, params.limit).await?;
    
    Ok(json_ok(result))
}

pub async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateUser>,
) -> Result<impl IntoResponse, AppError> {
    let oid = ObjectId::parse_str(&id).map_err(|_| AppError::ValidationError("Invalid ID format".into()))?;
    payload.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;

    let service = get_service(&state);
    service.update_user(oid, payload).await?;
    
    Ok(json_ok("User updated successfully"))
}
