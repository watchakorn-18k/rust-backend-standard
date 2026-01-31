#[cfg(test)]
mod tests {
    use fldp_rust_backend_template::services::user_service::{UserService, IUserService};
    use fldp_rust_backend_template::dtos::user::{CreateUser, UpdateUser};
    use fldp_rust_backend_template::models::user::User;
    use fldp_rust_backend_template::mock::repositories::user_repository_mock::MockUserRepository;
    use std::sync::Arc;
    use mockall::predicate::*;
    use chrono::Utc;
    use bcrypt::{hash, DEFAULT_COST};

    #[tokio::test]
    async fn test_create_user_logic() {
        let mut mock_repo = MockUserRepository::new();
        
        mock_repo.expect_find_by_email()
            .with(eq("test@example.com"))
            .times(1)
            .returning(|_| Ok(None));
            
        mock_repo.expect_create()
            .times(1)
            .returning(|_| Ok("mock_id".to_string()));
            
        let service = UserService::new(Arc::new(mock_repo));
        
        let input = CreateUser {
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        };
        
        let result = service.create_user(input).await;
        
        assert!(result.is_ok());
        let user = result.unwrap();
        assert_eq!(user.email, "test@example.com");
    }

    #[tokio::test]
    async fn test_get_user_success() {
        let mut mock_repo = MockUserRepository::new();
        let user_id = "user_123";
        let mock_user = User {
            id: Some(user_id.to_string()),
            username: "test".into(),
            email: "test@test.com".into(),
            password_hash: "hash".into(),
            role: "user".into(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        mock_repo.expect_find_by_id()
            .with(eq(user_id))
            .times(1)
            .returning(move |_| Ok(Some(mock_user.clone())));

        let service = UserService::new(Arc::new(mock_repo));
        let result = service.get_user(user_id).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().id, user_id);
    }

    #[tokio::test]
    async fn test_list_users() {
        let mut mock_repo = MockUserRepository::new();
        mock_repo.expect_find_all()
            .times(1)
            .returning(|_, _| Ok(vec![]));
        mock_repo.expect_count()
            .times(1)
            .returning(|| Ok(0));

        let service = UserService::new(Arc::new(mock_repo));
        let result = service.list_users(Some(1), Some(10)).await;

        assert!(result.is_ok());
        let paged = result.unwrap();
        assert_eq!(paged.total, 0);
        assert_eq!(paged.data.len(), 0);
    }

    #[tokio::test]
    async fn test_update_user_success() {
        let mut mock_repo = MockUserRepository::new();
        let user_id = "user_123";
        
        mock_repo.expect_update()
            .times(1)
            .returning(|_, _| Ok(()));

        let service = UserService::new(Arc::new(mock_repo));
        let input = UpdateUser {
            username: Some("newname".into()),
            email: None,
        };

        let result = service.update_user(user_id, input).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_authenticate_success() {
        let mut mock_repo = MockUserRepository::new();
        let email = "test@test.com";
        let password = "pass";
        let password_hash = hash(password, DEFAULT_COST).unwrap();

        let mock_user = User {
            id: Some("id".into()),
            username: "test".into(),
            email: email.into(),
            password_hash,
            role: "user".into(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        mock_repo.expect_find_by_email()
            .with(eq(email))
            .times(1)
            .returning(move |_| Ok(Some(mock_user.clone())));

        let service = UserService::new(Arc::new(mock_repo));
        let result = service.authenticate(email, password).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().email, email);
    }
}
