use std::sync::Arc;

use crate::{
    domain::user::repository::UserRepository,
    util::{
        error::{self, Error},
        hash::verify_password,
    },
};

use super::model::{LoginRequest, LoginResponse};

pub struct AuthService {
    user_repository: Arc<dyn UserRepository>,
}

impl AuthService {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> AuthService {
        AuthService { user_repository }
    }
}

impl AuthService {
    pub async fn login(&self, req: &LoginRequest) -> Result<LoginResponse, Error> {
        let user = match self.user_repository.get_by_email(&req.email).await {
            Ok(user) => user,
            Err(err) => {
                log::error!("failed to get user by email: {}", err);
                return Err(error::internal());
            }
        };

        let is_password_verified = verify_password(&user.password, &req.password)?;
        if !is_password_verified {
            return Err(error::unauthenticated());
        }

        Ok(LoginResponse { user })
    }
}
