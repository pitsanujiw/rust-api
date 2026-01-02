use uuid::Uuid;

use crate::domain::errors::DomainError;
use crate::domain::user::User;
use crate::usecase::ports::UserRepository;

pub struct UserService<R: UserRepository> {
    repo: R,
}

impl<R: UserRepository> UserService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn create_user(&self, username: String, email: String, active: Option<bool>) -> Result<User, DomainError> {
        if username.trim().is_empty() {
            return Err(DomainError::Validation("username is required".into()));
        }
        if !email.contains('@') {
            return Err(DomainError::Validation("email is invalid".into()));
        }
        self.repo.create(username, email, active.unwrap_or(true)).await
    }

    pub async fn get_user(&self, id: Uuid) -> Result<User, DomainError> {
        match self.repo.find_by_id(id).await? {
            Some(u) => Ok(u),
            None => Err(DomainError::NotFound),
        }
    }

    pub async fn list_users(&self, active: Option<bool>, limit: Option<i64>, offset: Option<i64>) -> Result<Vec<User>, DomainError> {
        let limit = limit.unwrap_or(50).clamp(1, 200);
        let offset = offset.unwrap_or(0).max(0);
        self.repo.list(active, limit, offset).await
    }

    pub async fn update_user(
        &self,
        id: Uuid,
        username: Option<String>,
        email: Option<String>,
        active: Option<bool>,
    ) -> Result<User, DomainError> {
        if let Some(e) = &email {
            if !e.contains('@') {
                return Err(DomainError::Validation("email is invalid".into()));
            }
        }

        match self.repo.update(id, username, email, active).await? {
            Some(u) => Ok(u),
            None => Err(DomainError::NotFound),
        }
    }

    pub async fn delete_user(&self, id: Uuid) -> Result<(), DomainError> {
        let deleted = self.repo.delete(id).await?;
        if !deleted {
            return Err(DomainError::NotFound);
        }
        Ok(())
    }
}
