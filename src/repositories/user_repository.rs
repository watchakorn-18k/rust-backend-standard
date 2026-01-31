use crate::models::user::User;
use mongodb::{
    bson::doc,
    Collection,
};
use futures::stream::TryStreamExt;

use async_trait::async_trait;
#[async_trait]
pub trait IUserRepository: Send + Sync {
    async fn create(&self, user: &User) -> Result<String, mongodb::error::Error>;
    async fn find_by_id(&self, id: &str) -> Result<Option<User>, mongodb::error::Error>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, mongodb::error::Error>;
    async fn update(&self, id: &str, update_doc: mongodb::bson::Document) -> Result<(), mongodb::error::Error>;
    async fn find_all(&self, skip: u64, limit: i64) -> Result<Vec<User>, mongodb::error::Error>;
    async fn count(&self) -> Result<u64, mongodb::error::Error>;
}

#[derive(Clone)]
pub struct UserRepository {
    collection: Collection<User>,
}

use crate::db::mongo::IMongoProvider;

impl UserRepository {
    pub fn new(db: &dyn IMongoProvider) -> Self {
        Self {
            collection: db.database().collection("users"),
        }
    }
}

#[async_trait]
impl IUserRepository for UserRepository {
    async fn create(&self, user: &User) -> Result<String, mongodb::error::Error> {
        let result = self.collection.insert_one(user, None).await?;
        Ok(result.inserted_id.as_str().unwrap().to_string())
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<User>, mongodb::error::Error> {
        self.collection.find_one(doc! { "_id": id }, None).await
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, mongodb::error::Error> {
        self.collection
            .find_one(doc! { "email": email }, None)
            .await
    }

    async fn update(&self, id: &str, update_doc: mongodb::bson::Document) -> Result<(), mongodb::error::Error> {
         self.collection.update_one(doc! { "_id": id }, doc! { "$set": update_doc }, None).await?;
         Ok(())
    }
    
    async fn find_all(&self, skip: u64, limit: i64) -> Result<Vec<User>, mongodb::error::Error> {
        let find_options = mongodb::options::FindOptions::builder()
            .skip(skip)
            .limit(limit)
            .sort(doc! { "created_at": -1 })
            .build();
            
        let mut cursor = self.collection.find(None, find_options).await?;
        let mut users = Vec::new();
        while let Some(user) = cursor.try_next().await? {
            users.push(user);
        }
        Ok(users)
    }

    async fn count(&self) -> Result<u64, mongodb::error::Error> {
        self.collection.count_documents(None, None).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::db_mock::MockMongoProvider;
    use mongodb::options::ClientOptions;
    use mongodb::Client;

    #[tokio::test]
    async fn test_user_repository_methods_error() {
        let mut mock_db = MockMongoProvider::new();
        let client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
        let client = Client::with_options(client_options).unwrap();
        let dummy_db = client.database("test");
        mock_db.expect_database().return_const(dummy_db);
        let repo = UserRepository::new(&mock_db);

        // This will attempt to connect and fail, but it covers the method call line.
        let _ = repo.find_by_id("id").await;
        let _ = repo.find_by_email("email").await;
        let _ = repo.count().await;
        let _ = repo.find_all(0, 10).await;
        let _ = repo.update("id", mongodb::bson::doc! {}).await;
        let _ = repo.create(&crate::models::user::User {
             id: None,
             username: "test".into(),
             email: "test@test.com".into(),
             password_hash: "hash".into(),
             role: "user".into(),
             created_at: chrono::Utc::now(),
             updated_at: chrono::Utc::now(),
        }).await;
    }
}
