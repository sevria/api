use chrono::{DateTime, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};

use crate::util::error::Error;

#[derive(Deserialize, Serialize)]
pub struct Claims {
    sub: String,
    exp: usize,
    iat: usize,
}

impl Claims {
    fn new(sub: String, expires_at: DateTime<Utc>) -> Claims {
        Self {
            sub,
            exp: expires_at.timestamp() as usize,
            iat: Utc::now().timestamp() as usize,
        }
    }
}

pub fn generate_token(
    secret: &str,
    user_id: &str,
    expires_at: &DateTime<Utc>,
) -> Result<String, Error> {
    let header = Header::default();
    let claims = Claims::new(user_id.to_string(), expires_at.to_owned());
    let key = EncodingKey::from_secret(secret.as_ref());

    let token = match encode(&header, &claims, &key) {
        Ok(token) => token,
        Err(err) => {
            log::error!("failed to generate token: {}", err);
            return Err(Error::Internal);
        }
    };

    Ok(token)
}
