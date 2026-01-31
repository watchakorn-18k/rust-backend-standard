pub mod repositories;
pub mod services;
pub mod db_mock;

pub fn get_mock_state() -> crate::state::AppState {
    use std::sync::Arc;
    use crate::state::InnerState;
    use crate::config::AppConfig;
    use crate::mock::db_mock::{MockMongoProvider, MockRedisProvider};
    use crate::mock::services::user_service_mock::MockUserService;
    use figment::Figment;
    use figment::providers::Serialized;

    let config: AppConfig = Figment::new()
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
        .extract()
        .unwrap();

    Arc::new(InnerState::new(
        Arc::new(MockMongoProvider::new()),
        config,
        Arc::new(MockRedisProvider::new()),
        Arc::new(MockUserService::new()),
    ))
}
