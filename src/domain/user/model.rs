use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

#[derive(Clone, FromRow, Serialize, ToSchema)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}
