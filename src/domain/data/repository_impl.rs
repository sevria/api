use std::sync::Arc;

use async_trait::async_trait;
use nanoid::nanoid;
use serde_json::Value;
use sqlx::{Pool, Postgres, QueryBuilder};

use crate::util::error::Error;

use super::{model::CreateDataRequest, repository::DataRepository};

pub struct DataRepositoryImpl {
    db: Arc<Pool<Postgres>>,
}

impl DataRepositoryImpl {
    pub fn new(db: Arc<Pool<Postgres>>) -> DataRepositoryImpl {
        DataRepositoryImpl { db }
    }
}

#[async_trait]
impl DataRepository for DataRepositoryImpl {
    async fn create(&self, data: &CreateDataRequest) -> Result<Value, Error> {
        let mut query = QueryBuilder::new("WITH inserted AS (INSERT INTO ");
        query.push(&data.schema_name);
        query.push(" (id)");
        query.push(" VALUES (");
        query.push_bind(nanoid!());
        query.push(") RETURNING *) SELECT ROW_TO_JSON(inserted) FROM inserted");

        match query.build_query_scalar().fetch_one(&*self.db).await {
            Ok(result) => Ok(result),
            Err(err) => {
                log::error!("Failed to create data: {}", err);
                Err(Error::Internal)
            }
        }
    }
}
