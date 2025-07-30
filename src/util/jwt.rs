use chrono::{DateTime, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};

use crate::util::error::{self, Error};

#[derive(Deserialize, Serialize)]
pub struct Claims {
    sub: i64,
    exp: usize,
    iat: usize,
}

impl Claims {
    fn new(sub: i64, expires_at: DateTime<Utc>) -> Claims {
        Self {
            sub,
            exp: expires_at.timestamp() as usize,
            iat: Utc::now().timestamp() as usize,
        }
    }
}

pub fn generate_token(
    secret: &str,
    user_id: i64,
    expires_at: DateTime<Utc>,
) -> Result<String, Error> {
    let header = Header::default();
    let claims = Claims::new(user_id, expires_at);
    let key = EncodingKey::from_secret(secret.as_ref());

    let token = match encode(&header, &claims, &key) {
        Ok(token) => token,
        Err(err) => {
            log::error!("failed to generate token: {}", err);
            return Err(error::internal());
        }
    };

    Ok(token)
}
