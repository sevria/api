use std::sync::Arc;

use async_trait::async_trait;
use sqlx::{Pool, Postgres, QueryBuilder};

use crate::{
    domain::field::{
        model::{Field, UpdateFieldRequest},
        repository::FieldRepository,
    },
    util::error::Error,
};

pub struct FieldRepositoryImpl {
    db: Arc<Pool<Postgres>>,
}

impl FieldRepositoryImpl {
    pub fn new(db: Arc<Pool<Postgres>>) -> FieldRepositoryImpl {
        FieldRepositoryImpl { db }
    }
}

#[async_trait]
impl FieldRepository for FieldRepositoryImpl {
    async fn create(&self, data: &Field) -> Result<Field, Error> {
        let mut query = QueryBuilder::new(
            "INSERT INTO fields (schema_id, name, value_type, required, default_value) VALUES (",
        );

        query.push_bind(&data.schema_id);
        query.push(", ");
        query.push_bind(&data.name);
        query.push(", ");
        query.push_bind(&data.value_type);
        query.push(", ");
        query.push_bind(&data.required);
        query.push(", ");
        query.push_bind(&data.default_value);
        query.push(") RETURNING *");

        match query.build_query_as::<Field>().fetch_one(&*self.db).await {
            Ok(field) => Ok(field),
            Err(err) => {
                log::error!("failed to create field: {}", err);
                Err(Error::Internal)
            }
        }
    }

    async fn list(&self, schema_id: &str) -> Result<Vec<Field>, Error> {
        let mut query = QueryBuilder::new("SELECT * FROM fields");

        query.push(" WHERE schema_id = ");
        query.push_bind(&schema_id);
        query.push(" ORDER BY name ASC");

        match query.build_query_as::<Field>().fetch_all(&*self.db).await {
            Ok(fields) => Ok(fields),
            Err(err) => {
                log::error!("failed to list fields: {}", err);
                return Err(Error::Internal);
            }
        }
    }

    async fn update(&self, data: &UpdateFieldRequest) -> Result<Field, Error> {
        let mut query = QueryBuilder::new("UPDATE fields SET");
        query.push(" name = ");
        query.push_bind(&data.name);

        if let Some(name) = &data.name {
            query.push(", name = ");
            query.push_bind(name);
        }

        query.push(" WHERE schema_id = ");
        query.push_bind(&data.schema_id);
        query.push(" AND name = ");
        query.push_bind(&data.name);
        query.push(" RETURNING *");

        match query.build_query_as::<Field>().fetch_one(&*self.db).await {
            Ok(field) => Ok(field),
            Err(err) => {
                log::error!("failed to update field: {}", err);
                return Err(Error::Internal);
            }
        }
    }

    async fn delete(&self, schema_id: &str, name: &str) -> Result<Field, Error> {
        let mut query = QueryBuilder::new("DELETE FROM fields WHERE schema_id = ");
        query.push_bind(&schema_id);
        query.push(" AND name = ");
        query.push_bind(name);
        query.push(" RETURNING *");

        match query.build_query_as::<Field>().fetch_one(&*self.db).await {
            Ok(field) => Ok(field),
            Err(err) => {
                log::error!("failed to delete field: {}", err);
                return Err(Error::Internal);
            }
        }
    }
}
