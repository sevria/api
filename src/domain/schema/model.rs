use fake::Dummy;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct Schema {
    pub id: String,
    pub name: String,
    pub fields: Vec<SchemaField>,
}

#[derive(Debug, Deserialize, Dummy, Serialize, ToSchema)]
pub struct SchemaField {
    pub name: String,
    pub value_type: String,
    pub required: bool,
    pub default_value: Option<String>,
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
