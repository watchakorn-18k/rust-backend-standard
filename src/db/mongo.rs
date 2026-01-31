use mongodb::{Database, Client, options::ClientOptions};
use std::ops::Deref;

#[derive(Clone)]
pub struct MongoProvider {
    db: Database,
}

impl MongoProvider {
    pub async fn new(uri: &str, db_name: &str) -> Result<Self, mongodb::error::Error> {
        let client_options = ClientOptions::parse(uri).await?;
        let client = Client::with_options(client_options)?;
        let db = client.database(db_name);
        Ok(Self { db })
    }
}

impl Deref for MongoProvider {
    type Target = Database;

    fn deref(&self) -> &Self::Target {
        &self.db
    }
}
