use async_trait::async_trait;

use crate::{domain::session::model::Session, util::error::Error};

#[async_trait]
pub trait SessionRepository: Send + Sync {
    async fn create(&self, data: &Session) -> Result<Session, Error>;
    async fn get(&self, token: &str, user_id: &str) -> Result<Session, Error>;
}
