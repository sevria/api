use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct Schema {
    pub id: String,
    pub name: String,
    pub fields: Vec<SchemaField>,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct SchemaField {
    pub name: String,
    pub value_type: String,
    pub required: bool,
    pub default_value: Option<Value>,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateSchemaRequest {
    pub id: String,
    pub name: String,
    pub fields: Vec<SchemaField>,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateSchemaRequest {
    pub id: Option<String>,
    pub name: Option<String>,
    pub fields: Option<Vec<SchemaField>>,
}
