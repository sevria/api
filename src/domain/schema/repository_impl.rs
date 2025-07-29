use std::sync::Arc;

use async_trait::async_trait;
use sqlx::{Pool, Postgres, QueryBuilder};

use crate::{
    domain::schema::{
        model::{Schema, UpdateSchemaRequest},
        repository::SchemaRepository,
    },
    util::{
        error::{self, Error},
        paginator::Paginated,
    },
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
        let mut query = QueryBuilder::new("INSERT INTO schemas (name) VALUES (");

        query.push_bind(&data.name);
        query.push(") RETURNING *");

        match query.build_query_as::<Schema>().fetch_one(&*self.db).await {
            Ok(schema) => Ok(schema),
            Err(err) => {
                log::error!("failed to create schema: {}", err);
                Err(error::internal_with_message(format!("{}", err).as_str()))
            }
        }
    }

    async fn list(&self) -> Result<Paginated<Schema>, Error> {
        let query = sqlx::query_as::<_, Schema>("SELECT * FROM schemas ORDER BY name ASC");

        let schemas = match query.fetch_all(&*self.db).await {
            Ok(schemas) => schemas,
            Err(err) => {
                log::error!("failed to get schemas: {}", err);
                return Err(error::internal());
            }
        };

        Ok(Paginated {
            data: schemas,
            page: 1,
            page_size: 1,
            total: 1,
        })
    }

    async fn get(&self, id: i64) -> Result<Schema, Error> {
        let query = sqlx::query_as::<_, Schema>("SELECT * FROM schemas WHERE id = $1").bind(id);

        match query.fetch_one(&*self.db).await {
            Ok(schema) => Ok(schema),
            Err(err) => {
                log::error!("failed to get schema: {}", err);
                return Err(error::internal());
            }
        }
    }

    async fn update(&self, data: &UpdateSchemaRequest) -> Result<Schema, Error> {
        let mut query = QueryBuilder::new("UPDATE schemas SET");
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
                log::error!("failed to update schema: {}", err);
                return Err(error::internal());
            }
        }
    }

    async fn delete(&self, id: i64) -> Result<Schema, Error> {
        let mut query = QueryBuilder::new("DELETE FROM schemas WHERE id = ");
        query.push_bind(&id);
        query.push(" RETURNING *");

        match query.build_query_as::<Schema>().fetch_one(&*self.db).await {
            Ok(schema) => Ok(schema),
            Err(err) => {
                log::error!("failed to delete schema: {}", err);
                return Err(error::internal());
            }
        }
    }
}
