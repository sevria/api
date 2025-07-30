use async_trait::async_trait;

use crate::util::error::Error;

use super::model::User;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, data: &User) -> Result<User, Error>;
    async fn get_by_email(&self, email: &str) -> Result<User, Error>;
}
