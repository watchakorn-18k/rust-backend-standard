use crate::repositories::user_repository::IUserRepository;
use crate::models::user::User;
use mockall::mock;
use async_trait::async_trait;

mock! {
    pub UserRepository {}
    #[async_trait]
    impl IUserRepository for UserRepository {
        async fn create(&self, user: &User) -> Result<String, mongodb::error::Error>;
        async fn find_by_id(&self, id: &str) -> Result<Option<User>, mongodb::error::Error>;
        async fn find_by_email(&self, email: &str) -> Result<Option<User>, mongodb::error::Error>;
        async fn update(&self, id: &str, update_doc: mongodb::bson::Document) -> Result<(), mongodb::error::Error>;
        async fn find_all(&self, skip: u64, limit: i64) -> Result<Vec<User>, mongodb::error::Error>;
        async fn count(&self) -> Result<u64, mongodb::error::Error>;
    }
}
