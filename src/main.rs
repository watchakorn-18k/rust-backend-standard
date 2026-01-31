use axum::{
    routing::get,
    http::{header::{CONTENT_TYPE, AUTHORIZATION}, Method, HeaderValue}
};
use dotenvy::dotenv;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::{
    catch_panic::CatchPanicLayer,
    cors::CorsLayer,
    trace::TraceLayer,
};
mod config;
mod db;
mod dtos;
mod error;
mod handlers;
mod middlewares;
mod models;
mod providers;
mod repositories;
mod routes;
mod services;
mod state;
mod utils;

use crate::{
    config::AppConfig,
    state::InnerState,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Load environment variables
    dotenv().ok();

    // 2. Set Timezone
    utils::time::set_global_timezone();

    // 3. Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_timer(utils::time::BangkokTimer)
        .init();

    // Load configuration
    let config = AppConfig::new()?;

    // Connect to Database
    let db = db::mongo::MongoProvider::new(&config.mongodb_uri, &config.mongodb_name).await?;

    // Connect to Redis
    let redis = db::redis::RedisProvider::new(
        &config.redis_host,
        config.redis_port,
        config.redis_password.clone(),
        config.redis_db,
    ).await?;

    // Initialize Repositories
    let user_repo = repositories::user_repository::UserRepository::new(&db);

    // Initialize Services
    let user_service = services::user_service::UserService::new(user_repo);

    // Create AppState
    let state = Arc::new(InnerState::new(db, config.clone(), redis, user_service));

    // Build Router
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([CONTENT_TYPE, AUTHORIZATION]);

    let mut app = routes::init_routes(state.clone())
        .route("/", get(|| async { 
            axum::Json(serde_json::json!({ 
                "message": "Welcome to fdlp Rust Backend Standard API", 
                "version": "0.1.0", 
                "docs": "/docs" 
            })) 
        }))
        .route("/health", get(handlers::health::health_check));

    // 8. Documentation routes (Only in development)
    if state.config.app_mode == "development" {
        app = app
            .route("/docs", get(handlers::docs::scalar_ui))
            .route("/swagger.yaml", get(handlers::docs::swagger_yaml))
            .route("/schema", get(handlers::docs::schema_html));
    }

    let app = app
        .layer(axum::middleware::from_fn(middlewares::logger::logger_middleware))
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .layer(CatchPanicLayer::new())
        .with_state(state);

    // 6. Serve
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    println!("\x1b[1;32m🚀 Server is running on http://localhost:{}\x1b[0m", config.port);
    tracing::info!("Listening on {}", addr);
    
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
