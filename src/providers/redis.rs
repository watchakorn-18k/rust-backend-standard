use redis::aio::MultiplexedConnection;
use redis::AsyncCommands;

#[derive(Clone)]
pub struct RedisProvider {
    conn: MultiplexedConnection,
}

impl RedisProvider {
    pub async fn new(redis_url: &str) -> Result<Self, redis::RedisError> {
        let client = redis::Client::open(redis_url)?;
        let conn = client.get_multiplexed_tokio_connection().await?;
        Ok(Self { conn })
    }

    pub async fn set(&self, key: &str, value: &str) -> Result<(), redis::RedisError> {
        let mut conn = self.conn.clone();
        conn.set::<&str, &str, ()>(key, value).await?;
        Ok(())
    }

    pub async fn get(&self, key: &str) -> Result<Option<String>, redis::RedisError> {
        let mut conn = self.conn.clone();
        let val: Option<String> = conn.get(key).await?;
        Ok(val)
    }
}
