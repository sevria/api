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
    pub async fn create(&self, req: &CreateSchemaRequest) -> Result<Schema, Error> {
        let data = Schema::new(req.name.clone());
        self.schema_repository.create(&data).await
    }

    pub async fn list(&self) -> Result<Paginated<Schema>, Error> {
        self.schema_repository.list().await
    }

    pub async fn get(&self, id: &str) -> Result<Schema, Error> {
        self.schema_repository.get(id).await
    }
}
