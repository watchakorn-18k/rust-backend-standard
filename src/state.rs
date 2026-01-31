use crate::config::AppConfig;
use crate::providers::redis::RedisProvider;
use mongodb::Database;
use std::sync::Arc;

pub struct InnerState {
    pub db: Database,
    pub config: AppConfig,
    pub redis: RedisProvider,
}

pub type AppState = Arc<InnerState>;

impl InnerState {
    pub fn new(db: Database, config: AppConfig, redis: RedisProvider) -> Self {
        Self { db, config, redis }
    }
}
