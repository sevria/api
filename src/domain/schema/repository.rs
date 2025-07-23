use async_trait::async_trait;

use crate::{domain::schema::model::Schema, util::error::Error};

#[async_trait]
pub trait SchemaRepository: Send + Sync {
    async fn create_schema(&self, schema: &Schema) -> Result<Schema, Error>;
}
