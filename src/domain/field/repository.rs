use async_trait::async_trait;

use crate::{
    domain::field::model::{Field, UpdateFieldRequest},
    util::error::Error,
};

#[async_trait]
pub trait FieldRepository: Send + Sync {
    async fn create(&self, data: &Field) -> Result<Field, Error>;
    async fn list(&self, schema_id: i64) -> Result<Vec<Field>, Error>;
    async fn update(&self, data: &UpdateFieldRequest) -> Result<Field, Error>;
    async fn delete(&self, schema_id: i64, name: &str) -> Result<Field, Error>;
}
