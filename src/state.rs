use crate::config::AppConfig;
use crate::db::{redis::IRedisProvider, mongo::IMongoProvider};
use crate::services::user_service::IUserService;
use std::sync::Arc;

pub struct InnerState {
    pub db: Arc<dyn IMongoProvider>,
    pub config: AppConfig,
    pub redis: Arc<dyn IRedisProvider>,
    pub user_service: Arc<dyn IUserService>,
}

pub type AppState = Arc<InnerState>;

impl InnerState {
    pub fn new(
        db: Arc<dyn IMongoProvider>,
        config: AppConfig,
        redis: Arc<dyn IRedisProvider>,
        user_service: Arc<dyn IUserService>,
    ) -> Self {
        Self { db, config, redis, user_service }
    }
}
