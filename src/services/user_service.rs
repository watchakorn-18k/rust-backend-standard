use crate::{
    dtos::user::{CreateUser, UpdateUser, UserResponse},
    error::AppError,
    models::user::User,
    repositories::user_repository::UserRepository,
    utils::pagination::PaginationResult,
};
use chrono::Utc;
use mongodb::bson::doc;
use bcrypt::{hash, DEFAULT_COST};

#[derive(Clone)]
pub struct UserService {
    repo: UserRepository,
}

impl UserService {
    pub fn new(repo: UserRepository) -> Self {
        Self { repo }
    }

    pub async fn create_user(&self, input: CreateUser) -> Result<UserResponse, AppError> {
        if self.repo.find_by_email(&input.email).await?.is_some() {
            return Err(AppError::ValidationError("Email already exists".into()));
        }

        let password_hash = hash(input.password, DEFAULT_COST)
            .map_err(|_| AppError::AuthError)?;

        let user_id = uuid::Uuid::new_v4().to_string();

        let user = User {
            id: Some(user_id.clone()),
            username: input.username,
            email: input.email,
            password_hash,
            role: "user".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        self.repo.create(&user).await?;
        
        Ok(user.into())
    }

    pub async fn get_user(&self, id: &str) -> Result<UserResponse, AppError> {
        let user = self.repo.find_by_id(id).await?.ok_or(AppError::NotFound)?;
        Ok(user.into())
    }

    pub async fn list_users(
        &self,
        page: Option<u64>,
        limit: Option<u64>,
    ) -> Result<PaginationResult<UserResponse>, AppError> {
        let page = page.unwrap_or(1);
        let limit = limit.unwrap_or(10);
        let skip = (page - 1) * limit;

        let users = self.repo.find_all(skip, limit as i64).await?;
        let total = self.repo.count().await?;

        let user_responses: Vec<UserResponse> = users.into_iter().map(Into::into).collect();

        Ok(PaginationResult::new(user_responses, page, limit, total))
    }
    
    pub async fn update_user(&self, id: &str, input: UpdateUser) -> Result<(), AppError> {
        let mut update_doc = doc! { "updated_at": Utc::now() };
        
        if let Some(username) = input.username {
            update_doc.insert("username", username);
        }
        
        if let Some(email) = input.email {
             if self.repo.find_by_email(&email).await?.is_some() {
                 return Err(AppError::ValidationError("Email already exists".into()));
             }
             update_doc.insert("email", email);
        }

        self.repo.update(id, update_doc).await.map_err(Into::into)
    }
}
