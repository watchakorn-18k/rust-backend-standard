use fldp_rust_backend_template::routes::init_routes;
use fldp_rust_backend_template::state::InnerState;
use fldp_rust_backend_template::config::AppConfig;
use fldp_rust_backend_template::mock::db_mock::{MockMongoProvider, MockRedisProvider};
use fldp_rust_backend_template::mock::services::user_service_mock::MockUserService;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;
use std::sync::Arc;
use figment::Figment;
use figment::providers::Serialized;
use mongodb::options::ClientOptions;
use mongodb::Client;
use mockall::predicate::*;

fn get_mock_config() -> AppConfig {
    Figment::new()
        .merge(Serialized::default("mongodb_uri", "uri"))
        .merge(Serialized::default("mongodb_name", "db"))
        .merge(Serialized::default("redis_host", "localhost"))
        .merge(Serialized::default("redis_port", 6379))
        .merge(Serialized::default("redis_db", 0))
        .merge(Serialized::default("jwt_secret", "secret"))
        .merge(Serialized::default("aws_region", "us-east-1"))
        .merge(Serialized::default("aws_access_key_id", "id"))
        .merge(Serialized::default("aws_secret_access_key", "key"))
        .merge(Serialized::default("aws_bucket_name", "bucket"))
        .merge(Serialized::default("firebase_credentials_file", "file"))
        .merge(Serialized::default("app_mode", "development"))
        .extract()
        .unwrap()
}

#[tokio::test]
async fn test_health_route() {
    let mut mock_mongo = MockMongoProvider::new();
    let mut mock_redis = MockRedisProvider::new();
    
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let dummy_db = client.database("test");
    mock_mongo.expect_database().return_const(dummy_db);
    
    mock_redis.expect_get()
        .with(eq("health_check_ping"))
        .returning(|_| Ok(None));

    let state = Arc::new(InnerState::new(
        Arc::new(mock_mongo),
        get_mock_config(),
        Arc::new(mock_redis),
        Arc::new(MockUserService::new()),
    ));

    let app = init_routes(state.clone()).with_state(state);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_api_v1_user_list_route_unauthorized() {
    let state = Arc::new(InnerState::new(
        Arc::new(MockMongoProvider::new()),
        get_mock_config(),
        Arc::new(MockRedisProvider::new()),
        Arc::new(MockUserService::new()),
    ));

    let app = init_routes(state.clone()).with_state(state);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/users")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Should be unauthorized because of auth_middleware
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_api_v1_auth_register_route() {
    let mut mock_user_service = MockUserService::new();
    
    // Set expectation for register
    use fldp_rust_backend_template::dtos::user::UserResponse;
    mock_user_service.expect_create_user()
        .returning(|input| Ok(UserResponse {
            id: "123".to_string(),
            username: input.username,
            email: input.email,
            role: "user".to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }));

    let state = Arc::new(InnerState::new(
        Arc::new(MockMongoProvider::new()),
        get_mock_config(),
        Arc::new(MockRedisProvider::new()),
        Arc::new(mock_user_service),
    ));

    let app = init_routes(state.clone()).with_state(state);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/auth/register")
                .header("Content-Type", "application/json")
                .body(Body::from(r#"{"username":"test","email":"test@test.com","password":"password123"}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_docs_routes() {
    let state = Arc::new(InnerState::new(
        Arc::new(MockMongoProvider::new()),
        get_mock_config(),
        Arc::new(MockRedisProvider::new()),
        Arc::new(MockUserService::new()),
    ));

    let app = init_routes(state.clone()).with_state(state);

    // Test /docs
    let response = app.clone()
        .oneshot(Request::builder().uri("/docs").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // Test /swagger.yaml
    let response = app.clone()
        .oneshot(Request::builder().uri("/swagger.yaml").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // Test /schema
    let response = app
        .oneshot(Request::builder().uri("/schema").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_root_route() {
    let state = Arc::new(InnerState::new(
        Arc::new(MockMongoProvider::new()),
        get_mock_config(),
        Arc::new(MockRedisProvider::new()),
        Arc::new(MockUserService::new()),
    ));

    let app = init_routes(state.clone()).with_state(state);

    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_ws_route() {
    let state = Arc::new(InnerState::new(
        Arc::new(MockMongoProvider::new()),
        get_mock_config(),
        Arc::new(MockRedisProvider::new()),
        Arc::new(MockUserService::new()),
    ));

    let app = init_routes(state.clone()).with_state(state);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/ws")
                .header("Host", "localhost")
                .header("Connection", "Upgrade")
                .header("Upgrade", "websocket")
                .header("Sec-WebSocket-Version", "13")
                .header("Sec-WebSocket-Key", "SGVsbG8sIHdvcmxkIQ==")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // It should return 101 Switching Protocols, but oneshot might return 426
    let status = response.status();
    assert!(status == StatusCode::SWITCHING_PROTOCOLS || status == StatusCode::UPGRADE_REQUIRED);
}
