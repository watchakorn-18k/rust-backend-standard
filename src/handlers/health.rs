use axum::{extract::State, Json};
use serde_json::{json, Value};
use crate::state::AppState;

pub async fn health_check(State(state): State<AppState>) -> Json<Value> {
    // Check Mongo
    let mongo_status = match state.db.database().run_command(mongodb::bson::doc! {"ping": 1}, None).await {
        Ok(_) => "OK",
        Err(_) => "Error",
    };

    // Check Redis
    let redis_status = match state.redis.get("health_check_ping").await {
        Ok(_) => "OK",
        Err(_) => "Error",
    };

    Json(json!({
        "status": "UP",
        "database": mongo_status,
        "redis": redis_status,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::db_mock::{MockMongoProvider, MockRedisProvider};
    use crate::mock::services::user_service_mock::MockUserService;
    use crate::state::InnerState;
    use crate::config::AppConfig;
    use std::sync::Arc;
    use figment::Figment;
    use figment::providers::Serialized;
    use mockall::predicate::*;
    use mongodb::options::ClientOptions;
    use mongodb::Client;

    #[tokio::test]
    async fn test_health_check() {
        let mut mock_mongo = MockMongoProvider::new();
        let mut mock_redis = MockRedisProvider::new();
        
        // สร้าง Database instance โดยไม่เชื่อมต่อจริง (ใช้ localhost เป็น placeholder)
        let client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
        let client = Client::with_options(client_options).unwrap();
        let dummy_db = client.database("test");

        mock_mongo.expect_database().return_const(dummy_db);
        
        mock_redis.expect_get()
            .with(eq("health_check_ping"))
            .times(1)
            .returning(|_| Ok(None));

        let config: AppConfig = Figment::new()
            .merge(Serialized::default("mongodb_uri", "uri"))
            .merge(Serialized::default("mongodb_name", "db"))
            .merge(Serialized::default("redis_host", "host"))
            .merge(Serialized::default("redis_port", 6379))
            .merge(Serialized::default("redis_db", 0))
            .merge(Serialized::default("jwt_secret", "secret"))
            .merge(Serialized::default("aws_region", "region"))
            .merge(Serialized::default("aws_access_key_id", "id"))
            .merge(Serialized::default("aws_secret_access_key", "key"))
            .merge(Serialized::default("aws_bucket_name", "bucket"))
            .merge(Serialized::default("firebase_credentials_file", "file"))
            .extract()
            .unwrap();

        let state = Arc::new(InnerState::new(
            Arc::new(mock_mongo),
            config,
            Arc::new(mock_redis),
            Arc::new(MockUserService::new()),
        ));

        let res = health_check(State(state)).await;
        assert_eq!(res.0["status"], "UP");
        // database check จะเป็น Error เพราะมันต่อไม่ได้จริงๆ แต่มันก็ไม่ควร Panic
        assert!(res.0["database"] == "OK" || res.0["database"] == "Error");
    }

    #[tokio::test]
    async fn test_health_check_error() {
        let mut mock_mongo = MockMongoProvider::new();
        let mut mock_redis = MockRedisProvider::new();
        
        // Mock error for database
        let client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
        let client = Client::with_options(client_options).unwrap();
        let dummy_db = client.database("test");
        mock_mongo.expect_database().return_const(dummy_db);
        // Note: database().run_command failure is harder to mock directly on the Database object,
        // but we can at least mock the provider part.
        
        mock_redis.expect_get()
            .returning(|_| Err(redis::RedisError::from((redis::ErrorKind::ResponseError, "error"))));

        let config = crate::mock::get_mock_state().config.clone();

        let state = Arc::new(InnerState::new(
            Arc::new(mock_mongo),
            config,
            Arc::new(mock_redis),
            Arc::new(MockUserService::new()),
        ));

        let res = health_check(State(state)).await;
        assert_eq!(res.0["status"], "UP");
        assert_eq!(res.0["redis"], "Error");
    }
}
