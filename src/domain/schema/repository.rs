use async_trait::async_trait;

use crate::{
    domain::schema::model::Schema,
    util::{error::Error, paginator::Paginated},
};

#[async_trait]
pub trait SchemaRepository: Send + Sync {
    async fn create_schema(&self, schema: &Schema) -> Result<Schema, Error>;
    async fn get_schemas(&self) -> Result<Paginated<Schema>, Error>;
    async fn get_schema(&self, id: &str) -> Result<Schema, Error>;
}
