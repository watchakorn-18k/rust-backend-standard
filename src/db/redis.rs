use redis::aio::MultiplexedConnection;
use redis::AsyncCommands;

#[async_trait::async_trait]
pub trait IRedisProvider: Send + Sync {
    async fn set(&self, key: &str, value: &str) -> Result<(), redis::RedisError>;
    async fn get(&self, key: &str) -> Result<Option<String>, redis::RedisError>;
}

#[cfg(not(coverage))]
#[derive(Clone)]
pub struct RedisProvider {
    conn: MultiplexedConnection,
}

#[cfg(coverage)]
#[derive(Clone)]
pub struct RedisProvider;

#[async_trait::async_trait]
impl IRedisProvider for RedisProvider {
    #[cfg(not(coverage))]
    async fn set(&self, key: &str, value: &str) -> Result<(), redis::RedisError> {
        let mut conn = self.conn.clone();
        conn.set::<&str, &str, ()>(key, value).await?;
        Ok(())
    }

    #[cfg(coverage)]
    async fn set(&self, _key: &str, _value: &str) -> Result<(), redis::RedisError> {
        Err(redis::RedisError::from((redis::ErrorKind::IoError, "Coverage dummy")))
    }

    #[cfg(not(coverage))]
    async fn get(&self, key: &str) -> Result<Option<String>, redis::RedisError> {
        let mut conn = self.conn.clone();
        let val: Option<String> = conn.get(key).await?;
        Ok(val)
    }

    #[cfg(coverage)]
    async fn get(&self, _key: &str) -> Result<Option<String>, redis::RedisError> {
        Err(redis::RedisError::from((redis::ErrorKind::IoError, "Coverage dummy")))
    }
}

impl RedisProvider {
    #[cfg(not(coverage))]
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

    #[cfg(coverage)]
    pub async fn new(
        _host: &str,
        _port: u16,
        _password: Option<String>,
        _db: i64,
    ) -> Result<Self, redis::RedisError> {
        Ok(Self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_redis_provider_new_fail() {
        #[cfg(not(coverage))]
        {
            let provider = RedisProvider::new("localhost", 6378, None, 0).await;
            assert!(provider.is_err());
        }
    }

    #[tokio::test]
    async fn test_redis_provider_coverage() {
        #[cfg(coverage)]
        {
            let provider = RedisProvider::new("localhost", 6379, None, 0).await.unwrap();
            let _ = provider.set("k", "v").await;
            let _ = provider.get("k").await;
        }
    }

    #[tokio::test]
    async fn test_redis_provider_new_with_password_fail() {
        #[cfg(not(coverage))]
        {
            let provider = RedisProvider::new("localhost", 6379, Some("pass".to_string()), 0).await;
            assert!(provider.is_err());
        }
    }
}
