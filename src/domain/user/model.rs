use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

#[derive(Clone, Deserialize, FromRow, Serialize, ToSchema)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    #[serde(skip)]
    pub password: String,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

pub enum GetUserRequest {
    Id(String),
    Email(String),
}
