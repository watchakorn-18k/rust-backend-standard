use crate::db::mongo::IMongoProvider;
use crate::db::redis::IRedisProvider;
use mongodb::Database;
use mockall::mock;
use async_trait::async_trait;

mock! {
    pub MongoProvider {}
    impl IMongoProvider for MongoProvider {
        fn database(&self) -> Database;
    }
}

mock! {
    pub RedisProvider {}
    #[async_trait]
    impl IRedisProvider for RedisProvider {
        async fn set(&self, key: &str, value: &str) -> Result<(), redis::RedisError>;
        async fn get(&self, key: &str) -> Result<Option<String>, redis::RedisError>;
    }
}
