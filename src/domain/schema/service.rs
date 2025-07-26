use std::sync::Arc;

use crate::{
    domain::schema::{
        model::{CreateSchemaRequest, Schema},
        repository::SchemaRepository,
    },
    util::{error::Error, paginator::Paginated},
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

    pub async fn get_schemas(&self) -> Result<Paginated<Schema>, Error> {
        self.schema_repository.get_schemas().await
    }

    pub async fn get_schema(&self, id: &str) -> Result<Schema, Error> {
        self.schema_repository.get_schema(id).await
    }
}
