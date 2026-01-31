use crate::config::AppConfig;
use crate::db::{redis::RedisProvider, mongo::MongoProvider};
use crate::services::user_service::UserService;
use std::sync::Arc;

pub struct InnerState {
    pub db: MongoProvider,
    pub config: AppConfig,
    pub redis: RedisProvider,
    pub user_service: UserService,
}

pub type AppState = Arc<InnerState>;

impl InnerState {
    pub fn new(
        db: MongoProvider,
        config: AppConfig,
        redis: RedisProvider,
        user_service: UserService,
    ) -> Self {
        Self { db, config, redis, user_service }
    }
}
