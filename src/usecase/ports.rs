use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::{errors::DomainError, user::User};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, username: String, email: String, active: bool) -> Result<User, DomainError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, DomainError>;
    async fn list(&self, active: Option<bool>, limit: i64, offset: i64) -> Result<Vec<User>, DomainError>;
    async fn update(
        &self,
        id: Uuid,
        username: Option<String>,
        email: Option<String>,
        active: Option<bool>,
    ) -> Result<Option<User>, DomainError>;
    async fn delete(&self, id: Uuid) -> Result<bool, DomainError>;
}
