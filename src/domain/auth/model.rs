use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::domain::user::model::User;

#[derive(Deserialize, Serialize, ToSchema)]
pub struct Token {
    pub token: String,
    #[schema(format = "date-time")]
    pub expires_at: DateTime<Utc>,
}

#[derive(Deserialize, ToSchema)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, ToSchema)]
pub struct LoginResponse {
    pub access: Token,
    pub refresh: Token,
    pub user: User,
}

#[derive(Deserialize, ToSchema)]
pub struct RefreshTokenRequest {
    pub token: String,
    pub user_id: i64,
}
