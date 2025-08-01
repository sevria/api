use async_trait::async_trait;

use crate::util::error::Error;

use super::model::{GetUserRequest, User};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, data: &User) -> Result<User, Error>;
    async fn get(&self, req: &GetUserRequest) -> Result<User, Error>;
}
