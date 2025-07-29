use std::sync::Arc;

use crate::{
    domain::user::repository::UserRepository,
    util::error::{self, Error},
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

        Ok(LoginResponse { user })
    }
}
