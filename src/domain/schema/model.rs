use fake::Dummy;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

#[derive(Clone, FromRow, Serialize, ToSchema)]
pub struct Schema {
    pub id: String,
    pub name: String,
    #[sqlx(skip)]
    pub fields: Vec<SchemaField>,
}

impl Schema {
    pub fn new(name: String, mut fields: Vec<SchemaField>) -> Schema {
        let schema_id = nanoid!();

        // Set the schema_id and generate id for each field
        for field in &mut fields {
            field.id = nanoid!();
            field.schema_id = schema_id.clone();
        }

        Schema {
            id: schema_id,
            name,
            fields,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Dummy, FromRow, Serialize, ToSchema)]
pub struct SchemaField {
    #[serde(skip)]
    pub id: String,
    #[serde(skip)]
    pub schema_id: String,
    pub name: String,
    pub value_type: String,
    pub required: bool,
    pub default_value: Option<String>,
}

impl SchemaField {
    pub fn new(
        schema_id: String,
        name: String,
        value_type: String,
        required: bool,
        default_value: Option<String>,
    ) -> SchemaField {
        SchemaField {
            id: nanoid!(),
            schema_id,
            name,
            value_type,
            required,
            default_value,
        }
    }
}

#[derive(Debug, Deserialize, Dummy, Serialize, ToSchema)]
pub struct CreateSchemaRequest {
    pub name: String,
    pub fields: Vec<SchemaField>,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateSchemaRequest {
    pub id: Option<String>,
    pub name: Option<String>,
    pub fields: Option<Vec<SchemaField>>,
}
