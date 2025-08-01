use std::sync::Arc;

use async_trait::async_trait;
use sqlx::{Pool, Postgres, QueryBuilder};

use crate::{
    domain::schema::{
        model::{Schema, UpdateSchemaRequest},
        repository::SchemaRepository,
    },
    util::{error::Error, paginator::Paginated},
};

pub struct SchemaRepositoryImpl {
    db: Arc<Pool<Postgres>>,
}

impl SchemaRepositoryImpl {
    pub fn new(db: Arc<Pool<Postgres>>) -> SchemaRepositoryImpl {
        SchemaRepositoryImpl { db }
    }
}

#[async_trait]
impl SchemaRepository for SchemaRepositoryImpl {
    async fn create(&self, data: &Schema) -> Result<Schema, Error> {
        match self.get_by_name(&data.name).await {
            Ok(_) => {
                return Err(Error::AlreadyPresent);
            }
            Err(Error::NotFound) => {}
            Err(err) => {
                return Err(err);
            }
        }

        let mut query = QueryBuilder::new("INSERT INTO sevria_schemas (id, name) VALUES (");

        query.push_bind(&data.id);
        query.push(", ");
        query.push_bind(&data.name);
        query.push(") RETURNING *");

        let schema = match query.build_query_as::<Schema>().fetch_one(&*self.db).await {
            Ok(schema) => schema,
            Err(err) => {
                log::error!("Failed to create schema: {}", err);
                return Err(Error::Internal);
            }
        };

        // Create table
        let mut query = QueryBuilder::new("CREATE TABLE ");
        query.push(&data.name);
        query.push(" (id CHAR(21) PRIMARY KEY)");

        match query.build().execute(&*self.db).await {
            Ok(_) => {}
            Err(err) => {
                log::error!("Failed to create table: {}", err);
                return Err(Error::Internal);
            }
        }

        Ok(schema)
    }

    async fn list(&self) -> Result<Paginated<Schema>, Error> {
        let query = sqlx::query_as::<_, Schema>("SELECT * FROM sevria_schemas ORDER BY name ASC");

        let schemas = match query.fetch_all(&*self.db).await {
            Ok(schemas) => schemas,
            Err(err) => {
                log::error!("Failed to get schemas: {}", err);
                return Err(Error::Internal);
            }
        };

        Ok(Paginated {
            data: schemas,
            page: 1,
            page_size: 1,
            total: 1,
        })
    }

    async fn get(&self, id: &str) -> Result<Schema, Error> {
        let query =
            sqlx::query_as::<_, Schema>("SELECT * FROM sevria_schemas WHERE id = $1").bind(id);

        match query.fetch_one(&*self.db).await {
            Ok(schema) => Ok(schema),
            Err(err) => {
                log::error!("Failed to get schema: {}", err);
                return Err(Error::Internal);
            }
        }
    }

    async fn get_by_name(&self, name: &str) -> Result<Schema, Error> {
        let query =
            sqlx::query_as::<_, Schema>("SELECT * FROM sevria_schemas WHERE name = $1").bind(name);

        match query.fetch_one(&*self.db).await {
            Ok(schema) => Ok(schema),
            Err(sqlx::Error::RowNotFound) => Err(Error::NotFound),
            Err(err) => {
                log::error!("Failed to get schema by name: {}", err);
                return Err(Error::Internal);
            }
        }
    }

    async fn update(&self, data: &UpdateSchemaRequest) -> Result<Schema, Error> {
        let mut query = QueryBuilder::new("UPDATE sevria_schemas SET");
        query.push(" id = ");
        query.push_bind(&data.id);

        if let Some(name) = &data.name {
            query.push(", name = ");
            query.push_bind(name);
        }

        query.push(" WHERE id = ");
        query.push_bind(&data.id);
        query.push(" RETURNING *");

        match query.build_query_as::<Schema>().fetch_one(&*self.db).await {
            Ok(schema) => Ok(schema),
            Err(err) => {
                log::error!("Failed to update schema: {}", err);
                return Err(Error::Internal);
            }
        }
    }

    async fn delete(&self, id: &str) -> Result<Schema, Error> {
        let mut query = QueryBuilder::new("DELETE FROM sevria_schemas WHERE id = ");
        query.push_bind(&id);
        query.push(" RETURNING *");

        match query.build_query_as::<Schema>().fetch_one(&*self.db).await {
            Ok(schema) => Ok(schema),
            Err(err) => {
                log::error!("Failed to delete schema: {}", err);
                return Err(Error::Internal);
            }
        }
    }
}
