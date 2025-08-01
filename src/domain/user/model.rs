use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use sqlx::prelude::{FromRow, Type};
use utoipa::ToSchema;
use validator::Validate;

use crate::util::{error::Error, hash::hash_password};

#[derive(Clone, Deserialize, Serialize, ToSchema, Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(rename_all = "snake_case", type_name = "varchar")]
pub enum UserStatus {
    Active,
    Inactive,
    PendingVerification,
}

#[derive(Clone, Deserialize, FromRow, Serialize, ToSchema)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    #[serde(skip)]
    pub password: String,
    pub status: UserStatus,
}

impl User {
    pub fn new(
        name: String,
        email: String,
        password: String,
        status: UserStatus,
    ) -> Result<Self, Error> {
        let user = User {
            id: nanoid!(),
            name,
            email,
            password: hash_password(&password)?,
            status,
        };

        Ok(user)
    }
}

#[derive(Deserialize, ToSchema, Validate)]
pub struct CreateUserRequest {
    #[validate(length(
        min = 3,
        max = 50,
        message = "Name must be between 3 and 50 characters"
    ))]
    pub name: String,

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

pub enum GetUserRequest {
    Id(String),
    Email(String),
}
