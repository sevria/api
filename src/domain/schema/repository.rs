use async_trait::async_trait;

use crate::{
    domain::schema::model::{Schema, UpdateSchemaRequest},
    util::{error::Error, paginator::Paginated},
};

#[async_trait]
pub trait SchemaRepository: Send + Sync {
    async fn create(&self, data: &Schema) -> Result<Schema, Error>;
    async fn list(&self) -> Result<Paginated<Schema>, Error>;
    async fn get(&self, id: i64) -> Result<Schema, Error>;
    async fn update(&self, data: &UpdateSchemaRequest) -> Result<Schema, Error>;
    async fn delete(&self, id: i64) -> Result<Schema, Error>;
}
