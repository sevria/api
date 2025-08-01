use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

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

#[derive(Deserialize, Serialize, ToSchema)]
pub struct LoginResponse {
    pub access: Token,
    pub refresh: Token,
    pub user: User,
}

#[derive(Deserialize, ToSchema, Validate)]
pub struct RefreshTokenRequest {
    #[validate(length(equal = 36, message = "Token must be 36 characters"))]
    pub token: String,
    #[validate(length(equal = 21, message = "User ID must be 21 characters"))]
    pub user_id: String,
}
