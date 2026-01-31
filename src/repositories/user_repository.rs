use crate::models::user::User;
use mongodb::{
    bson::{doc, oid::ObjectId},
    Collection, Database,
};
use futures::stream::TryStreamExt;

#[derive(Clone)]
pub struct UserRepository {
    collection: Collection<User>,
}

impl UserRepository {
    pub fn new(db: &Database) -> Self {
        Self {
            collection: db.collection("users"),
        }
    }

    pub async fn create(&self, user: &User) -> Result<ObjectId, mongodb::error::Error> {
        let result = self.collection.insert_one(user, None).await?;
        Ok(result.inserted_id.as_object_id().unwrap())
    }

    pub async fn find_by_id(&self, id: ObjectId) -> Result<Option<User>, mongodb::error::Error> {
        self.collection.find_one(doc! { "_id": id }, None).await
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, mongodb::error::Error> {
        self.collection
            .find_one(doc! { "email": email }, None)
            .await
    }

    pub async fn update(&self, id: ObjectId, update_doc: mongodb::bson::Document) -> Result<(), mongodb::error::Error> {
         self.collection.update_one(doc! { "_id": id }, doc! { "$set": update_doc }, None).await?;
         Ok(())
    }
    
    pub async fn find_all(&self, skip: u64, limit: i64) -> Result<Vec<User>, mongodb::error::Error> {
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

    pub async fn count(&self) -> Result<u64, mongodb::error::Error> {
        self.collection.count_documents(None, None).await
    }
}
