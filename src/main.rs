use axum::{
    routing::get,
    Router,
};
use dotenvy::dotenv;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

mod config;
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
    routes::user_routes::user_routes,
    state::InnerState,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    // 2. Load configuration
    dotenv().ok();
    let config = AppConfig::new()?;

    // 3. Connect to Database
    let client_options = mongodb::options::ClientOptions::parse(&config.mongodb_uri).await?;
    let client = mongodb::Client::with_options(client_options)?;
    let db = client.database(&config.mongodb_name);

    // Connect to Redis
    let redis_url = if let Some(ref pass) = config.redis_password {
        format!("redis://:{}@{}:{}/{}", pass, config.redis_host, config.redis_port, config.redis_db)
    } else {
        format!("redis://{}:{}/{}", config.redis_host, config.redis_port, config.redis_db)
    };
    let redis = providers::redis::RedisProvider::new(&redis_url).await?;

    // 4. Create AppState
    let state = Arc::new(InnerState::new(db, config.clone(), redis));

    // 5. Build Router
    let app = Router::new()
        .nest("/api/v1/users", user_routes(state.clone()))
        .route("/health", get(handlers::health::health_check))
        .route("/ws", get(handlers::ws::ws_handler))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    // 6. Serve
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    tracing::info!("Listening on {}", addr);
    
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
