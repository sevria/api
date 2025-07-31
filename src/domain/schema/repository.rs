use async_trait::async_trait;

use crate::{
    domain::schema::model::{Schema, UpdateSchemaRequest},
    util::{error::Error, paginator::Paginated},
};

#[async_trait]
pub trait SchemaRepository: Send + Sync {
    async fn create(&self, data: &Schema) -> Result<Schema, Error>;
    async fn list(&self) -> Result<Paginated<Schema>, Error>;
    async fn get(&self, id: &str) -> Result<Schema, Error>;
    async fn get_by_name(&self, name: &str) -> Result<Schema, Error>;
    async fn update(&self, data: &UpdateSchemaRequest) -> Result<Schema, Error>;
    async fn delete(&self, id: &str) -> Result<Schema, Error>;
}
