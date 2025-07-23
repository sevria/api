use std::sync::Arc;

use async_trait::async_trait;
use sqlx::{Pool, Postgres};

use crate::{
    domain::schema::{model::Schema, repository::SchemaRepository},
    util::error::{self, Error},
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

        // Insert schema
        let _: Result<(), Error> = match sqlx::query!(
            "INSERT INTO schemas (id, name) VALUES ($1, $2)",
            schema.id,
            schema.name
        )
        .execute(&mut *tx)
        .await
        {
            Ok(_) => Ok(()),
            Err(err) => {
                log::error!("failed to insert schema: {}", err);
                return Err(error::internal());
            }
        };

        // Insert each field
        for field in schema.fields.clone() {
            let _: Result<(), Error> = match sqlx::query!(
            "INSERT INTO schema_fields (id, schema_id, name, value_type, required, default_value)
             VALUES ($1, $2, $3, $4, $5, $6)",
            field.id,
            field.schema_id,
            field.name,
            field.value_type,
            field.required,
            field.default_value
        )
            .execute(&mut *tx)
            .await
            {
                Ok(_) => Ok(()),
                Err(err) => {
                    log::error!("failed to insert schema field: {}", err);
                    return Err(error::internal());
                }
            };
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
}
