use fldp_rust_backend_template::handlers::user_handler::UserHandler;
use fldp_rust_backend_template::mock::get_mock_state;
use fldp_rust_backend_template::mock::services::user_service_mock::MockUserService;
use fldp_rust_backend_template::dtos::user::{CreateUser, UserResponse, UpdateUser};
use fldp_rust_backend_template::state::InnerState;
use axum::extract::{State, Path, Query, Json};
use std::sync::Arc;
use mockall::predicate::*;
use chrono::Utc;
use fldp_rust_backend_template::utils::pagination::{PaginationParams, PaginationResult};

#[tokio::test]
async fn test_create_user_handler() {
    let mut mock_service = MockUserService::new();
    let input = CreateUser {
        username: "test".into(),
        email: "test@test.com".into(),
        password: "password".into(),
    };
    
    let response = UserResponse {
        id: "123".into(),
        username: "test".into(),
        email: "test@test.com".into(),
        role: "user".into(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    mock_service.expect_create_user()
        .times(1)
        .returning(move |_| Ok(response.clone()));

    let state = get_mock_state();
    // Reconstruct state with our mock service
    let state = Arc::new(InnerState::new(
        state.db.clone(),
        state.config.clone(),
        state.redis.clone(),
        Arc::new(mock_service),
    ));

    let res = UserHandler::create_user(State(state), Json(input)).await;
    assert!(res.is_ok());
}

#[tokio::test]
async fn test_get_user_handler() {
    let mut mock_service = MockUserService::new();
    let response = UserResponse {
        id: "123".into(),
        username: "test".into(),
        email: "test@test.com".into(),
        role: "user".into(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    mock_service.expect_get_user()
        .with(eq("123"))
        .times(1)
        .returning(move |_| Ok(response.clone()));

    let state = get_mock_state();
    let state = Arc::new(InnerState::new(
        state.db.clone(),
        state.config.clone(),
        state.redis.clone(),
        Arc::new(mock_service),
    ));

    let res = UserHandler::get_user(State(state), Path("123".into())).await;
    assert!(res.is_ok());
}

#[tokio::test]
async fn test_list_users_handler() {
    let mut mock_service = MockUserService::new();
    let result = PaginationResult {
        data: vec![],
        total: 0,
        page: 1,
        limit: 10,
        total_pages: 0,
    };

    mock_service.expect_list_users()
        .times(1)
        .returning(move |_, _| Ok(result.clone()));

    let state = get_mock_state();
    let state = Arc::new(InnerState::new(
        state.db.clone(),
        state.config.clone(),
        state.redis.clone(),
        Arc::new(mock_service),
    ));

    let params = PaginationParams { page: Some(1), limit: Some(10) };
    let res = UserHandler::list_users(State(state), Query(params)).await;
    assert!(res.is_ok());
}

#[tokio::test]
async fn test_update_user_handler() {
    let mut mock_service = MockUserService::new();
    mock_service.expect_update_user()
        .times(1)
        .returning(|_, _| Ok(()));

    let state = get_mock_state();
    let state = Arc::new(InnerState::new(
        state.db.clone(),
        state.config.clone(),
        state.redis.clone(),
        Arc::new(mock_service),
    ));

    let input = UpdateUser { username: Some("newname".into()), email: None };
    let res = UserHandler::update_user(State(state), Path("123".into()), Json(input)).await;
    assert!(res.is_ok());
}

#[tokio::test]
async fn test_get_user_not_found_handler() {
    let mut mock_service = MockUserService::new();
    mock_service.expect_get_user()
        .times(1)
        .returning(|_| Err(fldp_rust_backend_template::error::AppError::NotFound));

    let state = get_mock_state();
    let state = Arc::new(InnerState::new(
        state.db.clone(),
        state.config.clone(),
        state.redis.clone(),
        Arc::new(mock_service),
    ));

    let res = UserHandler::get_user(State(state), Path("nonexistent".into())).await;
    assert!(res.is_err());
}

#[tokio::test]
async fn test_update_user_handler_fail() {
    let mut mock_service = MockUserService::new();
    mock_service.expect_update_user()
        .returning(|_, _| Err(fldp_rust_backend_template::error::AppError::InternalServerError));

    let state = get_mock_state();
    let state = Arc::new(InnerState::new(
        state.db.clone(),
        state.config.clone(),
        state.redis.clone(),
        Arc::new(mock_service),
    ));

    let input = UpdateUser { username: Some("newname".into()), email: None };
    let res = UserHandler::update_user(State(state), Path("123".into()), Json(input)).await;
    assert!(res.is_err());
}

#[tokio::test]
async fn test_list_users_handler_fail() {
    let mut mock_service = MockUserService::new();
    mock_service.expect_list_users()
        .returning(|_, _| Err(fldp_rust_backend_template::error::AppError::InternalServerError));

    let state = get_mock_state();
    let state = Arc::new(InnerState::new(
        state.db.clone(),
        state.config.clone(),
        state.redis.clone(),
        Arc::new(mock_service),
    ));

    let params = PaginationParams { page: Some(1), limit: Some(10) };
    let res = UserHandler::list_users(State(state), Query(params)).await;
    assert!(res.is_err());
}

#[tokio::test]
async fn test_create_user_validation_error() {
    let state = get_mock_state();
    let payload = CreateUser {
        username: "u".to_string(), // too short
        email: "invalid".to_string(),
        password: "123".to_string(),
    };

    let res = UserHandler::create_user(State(state), Json(payload)).await;
    assert!(res.is_err());
}

#[tokio::test]
async fn test_update_user_validation_error() {
    let state = get_mock_state();
    let payload = UpdateUser {
        username: Some("u".to_string()), // too short
        email: None,
    };
    let res = UserHandler::update_user(State(state), Path("123".into()), Json(payload)).await;
    assert!(res.is_err());
}
