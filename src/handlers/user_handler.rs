use crate::{
    dtos::user::{CreateUser, UpdateUser},
    error::AppError,
    state::AppState,
    utils::response::{json_created, json_ok,},
    utils::pagination::PaginationParams,
};
use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    Json,
};
use validator::Validate;

pub struct UserHandler;

impl UserHandler {

    pub async fn create_user(
        State(state): State<AppState>,
        Json(payload): Json<CreateUser>,
    ) -> Result<impl IntoResponse, AppError> {
        payload.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;
        
        let user = state.user_service.create_user(payload).await?;
        Ok(json_created(user))
    }

    pub async fn get_user(
        State(state): State<AppState>,
        Path(id): Path<String>,
    ) -> Result<impl IntoResponse, AppError> {
        let user = state.user_service.get_user(&id).await?;
        Ok(json_ok(user))
    }

    pub async fn list_users(
        State(state): State<AppState>,
        Query(params): Query<PaginationParams>,
    ) -> Result<impl IntoResponse, AppError> {
        let result = state.user_service.list_users(params.page, params.limit).await?;
        Ok(json_ok(result))
    }

    pub async fn update_user(
        State(state): State<AppState>,
        Path(id): Path<String>,
        Json(payload): Json<UpdateUser>,
    ) -> Result<impl IntoResponse, AppError> {
        payload.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;

        state.user_service.update_user(&id, payload).await?;
        Ok(json_ok("User updated successfully"))
    }
}
