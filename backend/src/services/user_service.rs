use async_trait::async_trait;
use std::sync::Arc;
use std::time::Duration;

use crate::domain::{User, GlobalRole};
use crate::infra::errors::{AppError, AppResult};
use crate::infra::repositories::UserRepository;
use crate::auth::{hash_password, validate_password, verify_password};
use crate::infra::security::generate_token;
use crate::utils::{Id, generate_id};

#[async_trait]
pub trait UserService: Send + Sync {
    async fn authenticate(&self, email: &str, password: &str) -> AppResult<User>;
    async fn register(&self, email: String, name: String, password: String) -> AppResult<(User, String)>;
    async fn get_by_id(&self, user_id: Id) -> AppResult<User>;
    async fn get_all(&self) -> AppResult<Vec<User>>;
    async fn promote_to_teacher(&self, user_id: Id) -> AppResult<User>;
    async fn update_email_notifications(&self, user_id: Id, enabled: bool) -> AppResult<User>;
}

pub struct UserServiceImpl {
    user_repo: Arc<dyn UserRepository>,
}

impl UserServiceImpl {
    pub fn new(user_repo: Arc<dyn UserRepository>) -> Self {
        Self { user_repo }
    }
}

#[async_trait]
impl UserService for UserServiceImpl {
    async fn authenticate(&self, email: &str, password: &str) -> AppResult<User> {
        let result = self.user_repo
            .find_by_email(email)
            .await
            .map_err(AppError::Internal)?;

        let (user, password_hash) = result.ok_or_else(|| {
            AppError::Validation("Invalid email or password".into())
        })?;

        if password_hash.is_empty() {
            return Err(AppError::Internal(anyhow::anyhow!("User has no password hash set")));
        }

        if !verify_password(&password_hash, password)? {
            return Err(AppError::Validation("Invalid email or password".into()));
        }

        Ok(user)
    }

    async fn register(&self, email: String, name: String, password: String) -> AppResult<(User, String)> {
        validate_password(&password)?;

        let existing = self.user_repo
            .find_by_email(&email)
            .await
            .map_err(AppError::Internal)?;

        if existing.is_some() {
            return Err(AppError::Validation(format!("User with email {} already exists", email)));
        }

        let password_hash = hash_password(&password)?;
        let id = generate_id();

        let user = User {
            id,
            email,
            name: name.clone(),
            global_role: GlobalRole::Student,
            email_notifications_enabled: true,
        };

        self.user_repo
            .insert(&user, &password_hash)
            .await
            .map_err(AppError::Internal)?;

        let token = generate_token(user.id, &user.global_role.to_string(), name, Duration::from_secs(86400))?;

        Ok((user, token))
    }

    async fn get_by_id(&self, user_id: Id) -> AppResult<User> {
        self.user_repo
            .find_by_id(user_id)
            .await
            .map_err(AppError::Internal)?
            .ok_or_else(|| AppError::NotFound(format!("User {} not found", user_id)))
    }

    async fn get_all(&self) -> AppResult<Vec<User>> {
        self.user_repo
            .find_all()
            .await
            .map_err(AppError::Internal)
    }

    async fn promote_to_teacher(&self, user_id: Id) -> AppResult<User> {
        let updated = self.user_repo
            .update_global_role(user_id, GlobalRole::Teacher)
            .await
            .map_err(AppError::Internal)?;

        if !updated {
            return Err(AppError::NotFound(format!("User {} not found", user_id)));
        }

        self.get_by_id(user_id).await
    }

    async fn update_email_notifications(&self, user_id: Id, enabled: bool) -> AppResult<User> {
        let updated = self.user_repo
            .update_email_notifications(user_id, enabled)
            .await
            .map_err(AppError::Internal)?;

        if !updated {
            return Err(AppError::NotFound(format!("User {} not found", user_id)));
        }

        self.get_by_id(user_id).await
    }
}
