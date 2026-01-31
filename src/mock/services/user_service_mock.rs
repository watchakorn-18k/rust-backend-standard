use crate::services::user_service::IUserService;
use crate::dtos::user::{CreateUser, UpdateUser, UserResponse};
use crate::models::user::User;
use crate::error::AppError;
use crate::utils::pagination::PaginationResult;
use mockall::mock;
use async_trait::async_trait;

mock! {
    pub UserService {}
    #[async_trait]
    impl IUserService for UserService {
        async fn create_user(&self, input: CreateUser) -> Result<UserResponse, AppError>;
        async fn get_user(&self, id: &str) -> Result<UserResponse, AppError>;
        async fn list_users(&self, page: Option<u64>, limit: Option<u64>) -> Result<PaginationResult<UserResponse>, AppError>;
        async fn update_user(&self, id: &str, input: UpdateUser) -> Result<(), AppError>;
        async fn authenticate(&self, email: &str, password: &str) -> Result<User, AppError>;
    }
}
