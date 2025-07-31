use std::sync::Arc;

use chrono::{Duration, Utc};

use crate::{
    config::Config,
    domain::{
        auth::model::{RefreshTokenRequest, Token},
        session::{model::Session, repository::SessionRepository},
        user::repository::UserRepository,
    },
    util::{error::Error, hash::verify_password, jwt::generate_token},
};

use super::model::{LoginRequest, LoginResponse};

pub struct AuthService {
    config: Arc<Config>,
    session_repository: Arc<dyn SessionRepository>,
    user_repository: Arc<dyn UserRepository>,
}

impl AuthService {
    pub fn new(
        config: Arc<Config>,
        session_repository: Arc<dyn SessionRepository>,
        user_repository: Arc<dyn UserRepository>,
    ) -> AuthService {
        AuthService {
            config,
            session_repository,
            user_repository,
        }
    }
}

impl AuthService {
    pub async fn login(&self, req: &LoginRequest) -> Result<LoginResponse, Error> {
        let user = match self.user_repository.get_by_email(&req.email).await {
            Ok(user) => user,
            Err(err) => {
                log::error!("failed to get user by email: {}", err);
                return Err(Error::Internal);
            }
        };

        let is_password_verified = verify_password(&user.password, &req.password)?;
        if !is_password_verified {
            return Err(Error::Unauthenticated);
        }

        let access_token_expires_at =
            Utc::now() + Duration::minutes(self.config.jwt_expires_in_minutes);
        let access_token =
            generate_token(&self.config.jwt_secret, &user.id, access_token_expires_at)?;

        let session = Session::new(&user.id, Utc::now() + Duration::days(30));
        self.session_repository.create(&session).await?;

        Ok(LoginResponse {
            access: Token {
                token: access_token,
                expires_at: access_token_expires_at,
            },
            refresh: Token {
                token: session.token,
                expires_at: session.expires_at,
            },
            user,
        })
    }

    pub async fn refresh(&self, _: &RefreshTokenRequest) -> Result<LoginResponse, Error> {
        todo!()
    }
}
