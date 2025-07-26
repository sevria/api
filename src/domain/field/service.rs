use std::sync::Arc;

use crate::{
    domain::field::{
        model::{CreateFieldRequest, Field},
        repository::FieldRepository,
    },
    util::error::Error,
};

pub struct FieldService {
    field_repository: Arc<dyn FieldRepository>,
}

impl FieldService {
    pub fn new(field_repository: Arc<dyn FieldRepository>) -> FieldService {
        FieldService { field_repository }
    }
}

impl FieldService {
    pub async fn create(&self, req: &CreateFieldRequest) -> Result<Field, Error> {
        let data = Field::new(
            req.schema_id,
            req.name.clone(),
            req.value_type.clone(),
            req.required,
            req.default_value.clone(),
        );
        self.field_repository.create(&data).await
    }

    pub async fn list(&self, schema_id: i64) -> Result<Vec<Field>, Error> {
        self.field_repository.list(schema_id).await
    }
}
