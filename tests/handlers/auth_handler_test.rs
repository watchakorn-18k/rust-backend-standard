use fldp_rust_backend_template::handlers::auth_handler::{AuthHandler, LoginRequest};
use fldp_rust_backend_template::mock::get_mock_state;
use fldp_rust_backend_template::mock::services::user_service_mock::MockUserService;
use fldp_rust_backend_template::dtos::user::{CreateUser, UserResponse};
use fldp_rust_backend_template::state::InnerState;
use fldp_rust_backend_template::models::user::User;
use axum::extract::{State, Json};
use std::sync::Arc;
use mockall::predicate::*;
use chrono::Utc;

#[tokio::test]
async fn test_register_handler_success() {
    let mut mock_service = MockUserService::new();
    let input = CreateUser {
        username: "test".into(),
        email: "test@test.com".into(),
        password: "password123".into(),
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
    let state = Arc::new(InnerState::new(
        state.db.clone(),
        state.config.clone(),
        state.redis.clone(),
        Arc::new(mock_service),
    ));

    let res = AuthHandler::register(State(state), Json(input)).await;
    assert!(res.is_ok());
}

#[tokio::test]
async fn test_login_handler_success() {
    let mut mock_service = MockUserService::new();
    let payload = LoginRequest {
        email: "test@test.com".into(),
        password: "password123".into(),
    };

    let mock_user = User {
        id: Some("123".into()),
        username: "test".into(),
        email: "test@test.com".into(),
        password_hash: "hash".into(),
        role: "user".into(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    mock_service.expect_authenticate()
        .with(eq("test@test.com"), eq("password123"))
        .times(1)
        .returning(move |_, _| Ok(mock_user.clone()));

    let state = get_mock_state();
    let state = Arc::new(InnerState::new(
        state.db.clone(),
        state.config.clone(),
        state.redis.clone(),
        Arc::new(mock_service),
    ));

    let res = AuthHandler::login(State(state), Json(payload)).await;
    assert!(res.is_ok());
    let auth_res = res.unwrap();
    assert!(!auth_res.token.is_empty());
}

#[tokio::test]
async fn test_register_handler_validation_error() {
    let input = CreateUser {
        username: "t".into(),
        email: "invalid-email".into(),
        password: "123".into(),
    };

    let state = get_mock_state();
    let res = AuthHandler::register(State(state), Json(input)).await;
    assert!(res.is_err());
}

#[tokio::test]
async fn test_login_handler_validation_error() {
    let payload = LoginRequest {
        email: "invalid".into(),
        password: "123".into(),
    };

    let state = get_mock_state();
    let res = AuthHandler::login(State(state), Json(payload)).await;
    assert!(res.is_err());
}

#[tokio::test]
async fn test_login_handler_fail() {
    let mut mock_service = MockUserService::new();
    mock_service.expect_authenticate()
        .returning(|_, _| Err(fldp_rust_backend_template::error::AppError::InvalidCredentials));

    let state = get_mock_state();
    let state = Arc::new(InnerState::new(
        state.db.clone(),
        state.config.clone(),
        state.redis.clone(),
        Arc::new(mock_service),
    ));

    let payload = LoginRequest {
        email: "test@test.com".into(),
        password: "wrong".into(),
    };

    let res = AuthHandler::login(State(state), Json(payload)).await;
    assert!(res.is_err());
}
