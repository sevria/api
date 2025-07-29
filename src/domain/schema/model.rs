use fake::Dummy;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

use crate::domain::field::model::Field;

#[derive(Clone, FromRow, Serialize, ToSchema)]
pub struct Schema {
    pub id: i64,
    pub name: String,
    #[sqlx(skip)]
    pub fields: Vec<Field>,
}

impl Schema {
    pub fn new(name: String) -> Schema {
        Schema {
            id: 0,
            name,
            fields: vec![],
        }
    }
}

#[derive(Debug, Deserialize, Dummy, Serialize, ToSchema)]
pub struct CreateSchemaRequest {
    pub name: String,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateSchemaRequest {
    pub id: Option<i64>,
    pub name: Option<String>,
}
