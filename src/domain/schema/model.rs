use std::sync::LazyLock;

use fake::Dummy;
use nanoid::nanoid;
use regex::Regex;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;
use validator::Validate;

use crate::domain::field::model::Field;

#[derive(Clone, FromRow, Serialize, ToSchema)]
pub struct Schema {
    pub id: String,
    pub name: String,
    #[sqlx(skip)]
    pub fields: Vec<Field>,
}

impl Schema {
    pub fn new(name: String) -> Schema {
        Schema {
            id: nanoid!(),
            name,
            fields: vec![],
        }
    }
}

static NAME_SANITIZER: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-z][a-z0-9_]*$").unwrap());

#[derive(Debug, Deserialize, Dummy, Serialize, ToSchema, Validate)]
pub struct CreateSchemaRequest {
    #[validate(
        length(
            min = 1,
            max = 50,
            message = "Name must be between 1 and 50 characters"
        ), 
        regex(path = *NAME_SANITIZER, message = "Name must be lowercase and contain only letters, numbers, and underscores")
    )]
    pub name: String,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateSchemaRequest {
    pub id: Option<i64>,
    pub name: Option<String>,
}
