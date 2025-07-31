use fake::Dummy;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

#[derive(Clone, Debug, Deserialize, Dummy, FromRow, Serialize, ToSchema)]
pub struct Field {
    #[serde(skip)]
    pub schema_id: String,
    pub name: String,
    pub value_type: String,
    pub required: bool,
    pub default_value: Option<String>,
}

impl Field {
    pub fn new(
        schema_id: String,
        name: String,
        value_type: String,
        required: bool,
        default_value: Option<String>,
    ) -> Field {
        Field {
            schema_id,
            name,
            value_type,
            required,
            default_value,
        }
    }
}

#[derive(Deserialize, Dummy, Serialize, ToSchema)]
pub struct CreateFieldRequest {
    #[serde(skip)]
    pub schema_id: String,
    pub name: String,
    pub value_type: String,
    pub required: bool,
    pub default_value: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateFieldRequest {
    pub schema_id: Option<String>,
    pub name: Option<String>,
    pub value_type: Option<String>,
    pub required: Option<bool>,
    pub default_value: Option<String>,
}
