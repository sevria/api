use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::domain::user::model::User;

#[derive(Deserialize, ToSchema)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, ToSchema)]
pub struct LoginResponse {
    pub user: User,
}
