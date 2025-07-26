use std::sync::Arc;

use async_trait::async_trait;
use sqlx::{Pool, Postgres};

use crate::{
    domain::schema::{model::Schema, repository::SchemaRepository},
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
    async fn create_schema(&self, schema: &Schema) -> Result<Schema, Error> {
        let mut tx = self.db.begin().await.unwrap();

        let create_schema_query = sqlx::query!(
            "INSERT INTO schemas (id, name) VALUES ($1, $2)",
            schema.id,
            schema.name
        );

        if let Err(err) = create_schema_query.execute(&mut *tx).await {
            log::error!("failed to insert schema: {}", err);
            return Err(error::internal());
        }

        for field in schema.fields.clone() {
            let create_field_query = sqlx::query!(
                "INSERT INTO schema_fields (id, schema_id, name, value_type, required, default_value)
                VALUES ($1, $2, $3, $4, $5, $6)",
                field.id,
                field.schema_id,
                field.name,
                field.value_type,
                field.required,
                field.default_value
            );

            if let Err(err) = create_field_query.execute(&mut *tx).await {
                log::error!("failed to insert schema field: {}", err);
                return Err(error::internal());
            }
        }

        let _: Result<(), Error> = match tx.commit().await {
            Ok(_) => Ok(()),
            Err(err) => {
                log::error!("failed to commit transaction: {}", err);
                return Err(error::internal());
            }
        };

        Ok(schema.clone())
    }

    async fn get_schemas(&self) -> Result<Paginated<Schema>, Error> {
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

    async fn get_schema(&self, id: &str) -> Result<Schema, Error> {
        let query = sqlx::query_as::<_, Schema>("SELECT * FROM schemas WHERE id = $1").bind(id);

        let schema = match query.fetch_one(&*self.db).await {
            Ok(schema) => schema,
            Err(err) => {
                log::error!("failed to get schema: {}", err);
                return Err(error::internal());
            }
        };

        Ok(schema)
    }
}
