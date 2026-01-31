use mongodb::{Database, Client, options::ClientOptions};

pub trait IMongoProvider: Send + Sync {
    fn database(&self) -> Database;
}

#[derive(Clone)]
pub struct MongoProvider {
    db: Database,
}

impl IMongoProvider for MongoProvider {
    fn database(&self) -> Database {
        self.db.clone()
    }
}

impl MongoProvider {
    pub async fn new(uri: &str, db_name: &str) -> Result<Self, mongodb::error::Error> {
        let client_options = ClientOptions::parse(uri).await?;
        let client = Client::with_options(client_options)?;
        let db = client.database(db_name);
        Ok(Self { db })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mongo_provider_new() {
        let uri = "mongodb://localhost:27017";
        let db_name = "test";
        let provider = MongoProvider::new(uri, db_name).await;
        assert!(provider.is_ok());
    }

    #[tokio::test]
    async fn test_mongo_provider_database() {
        let uri = "mongodb://localhost:27017";
        let db_name = "test";
        let provider = MongoProvider::new(uri, db_name).await.unwrap();
        let _db = provider.database();
    }
}
