use std::sync::Arc;

use crate::{
    domain::schema::{
        model::{CreateSchemaRequest, Schema},
        repository::SchemaRepository,
    },
    util::error::Error,
};

pub struct SchemaService {
    schema_repository: Arc<dyn SchemaRepository>,
}

impl SchemaService {
    pub fn new(schema_repository: Arc<dyn SchemaRepository>) -> SchemaService {
        SchemaService { schema_repository }
    }
}

impl SchemaService {
    pub async fn create_schema(&self, req: &CreateSchemaRequest) -> Result<Schema, Error> {
        let schema = Schema::new(req.name.clone(), req.fields.clone());
        self.schema_repository.create_schema(&schema).await
    }
}
