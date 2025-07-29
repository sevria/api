use serde::Serialize;
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

#[derive(Clone, FromRow, Serialize, ToSchema)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub password: String,
}
