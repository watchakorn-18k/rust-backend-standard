use dotenvy::dotenv;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use fldp_rust_backend_template::{
    config::AppConfig,
    state::InnerState,
    utils,
    db,
    repositories,
    services,
    create_app,
};

#[cfg(not(coverage))]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenv().ok();

    // Set Timezone
    utils::time::set_global_timezone();

    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_timer(utils::time::BangkokTimer)
        .init();

    // Load configuration
    let config = AppConfig::new()?;

    // Connect to Database
    let db = Arc::new(db::mongo::MongoProvider::new(&config.mongodb_uri, &config.mongodb_name).await?);

    // Connect to Redis
    let redis = Arc::new(db::redis::RedisProvider::new(
        &config.redis_host,
        config.redis_port,
        config.redis_password.clone(),
        config.redis_db,
    ).await?);

    // Initialize Repositories
    let user_repo = Arc::new(repositories::user_repository::UserRepository::new(db.as_ref()));

    // Initialize Services
    let user_service = Arc::new(services::user_service::UserService::new(user_repo));

    // Create AppState
    let state = Arc::new(InnerState::new(db, config.clone(), redis, user_service));

    // Build Router
    let app = create_app(state);

    // 6. Serve
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    println!("\x1b[1;32m🚀 Server is running on http://localhost:{}\x1b[0m", config.port);
    tracing::info!("Listening on {}", addr);
    
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

#[cfg(coverage)]
fn main() {}
