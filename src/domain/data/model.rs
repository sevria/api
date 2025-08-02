use serde::Deserialize;
use serde_json::Value;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Deserialize, ToSchema, Validate)]
pub struct CreateDataRequest {
    #[serde(skip)]
    #[validate(custom(function = "crate::domain::schema::validate::name"))]
    pub schema_name: String,
    #[validate(custom(function = "super::validate::data"))]
    pub data: Value,
}
