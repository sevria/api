use chrono::{DateTime, Utc};
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

#[derive(Clone, FromRow, Serialize, ToSchema)]
pub struct Session {
    pub id: String,
    pub token: String,
    pub user_id: String,
    #[schema(format = "date-time")]
    pub created_at: DateTime<Utc>,
    #[schema(format = "date-time")]
    pub updated_at: DateTime<Utc>,
    #[schema(format = "date-time")]
    pub expires_at: DateTime<Utc>,
}

impl Session {
    pub fn new(user_id: String, expires_at: DateTime<Utc>) -> Session {
        Session {
            id: nanoid!(),
            token: nanoid!(36),
            user_id,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            expires_at,
        }
    }

    pub fn generate_token() -> String {
        nanoid!(36)
    }
}

#[derive(Deserialize, ToSchema)]
pub struct CreateSessionRequest {
    pub user_id: String,
    pub expires_at: DateTime<Utc>,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateSessionRequest {
    pub token: Option<String>,
}
