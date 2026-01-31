use redis::aio::MultiplexedConnection;
use redis::AsyncCommands;

#[derive(Clone)]
pub struct RedisProvider {
    conn: MultiplexedConnection,
}

impl RedisProvider {
    pub async fn new(
        host: &str,
        port: u16,
        password: Option<String>,
        db: i64,
    ) -> Result<Self, redis::RedisError> {
        let redis_url = match password {
            Some(ref pass) if !pass.is_empty() => {
                format!("redis://:{}@{}:{}/{}", pass, host, port, db)
            }
            _ => format!("redis://{}:{}/{}", host, port, db),
        };
        
        let client = redis::Client::open(redis_url)?;
        let conn = client.get_multiplexed_tokio_connection().await?;
        Ok(Self { conn })
    }

    #[allow(dead_code)]
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
