use chrono::{DateTime, Utc};
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

#[derive(FromRow, Serialize, ToSchema)]
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
    pub fn new(user_id: &str, expires_at: DateTime<Utc>) -> Session {
        Session {
            id: nanoid!(),
            token: nanoid!(36),
            user_id: user_id.to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            expires_at,
        }
    }
}

#[derive(Deserialize, ToSchema)]
pub struct CreateSessionRequest {
    pub user_id: String,
    pub expires_at: DateTime<Utc>,
}
