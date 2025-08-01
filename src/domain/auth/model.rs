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

#[derive(Deserialize, ToSchema, Validate)]
pub struct LoginRequest {
    #[validate(
        length(
            min = 3,
            max = 50,
            message = "Email must be between 3 and 50 characters"
        ),
        email(message = "Invalid email")
    )]
    pub email: String,
    #[validate(length(
        min = 8,
        max = 50,
        message = "Password must be between 8 and 50 characters"
    ))]
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
