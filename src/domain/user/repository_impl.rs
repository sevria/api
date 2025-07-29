use std::sync::Arc;

use async_trait::async_trait;
use sqlx::{Pool, Postgres};

use crate::util::error::{self, Error};

use super::{model::User, repository::UserRepository};

pub struct UserRepositoryImpl {
    db: Arc<Pool<Postgres>>,
}

impl UserRepositoryImpl {
    pub fn new(db: Arc<Pool<Postgres>>) -> UserRepositoryImpl {
        UserRepositoryImpl { db }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn get_by_email(&self, email: &str) -> Result<User, Error> {
        let query = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1").bind(email);

        match query.fetch_one(&*self.db).await {
            Ok(user) => Ok(user),
            Err(err) => {
                log::error!("failed to get user by email: {}", err);
                return Err(error::internal());
            }
        }
    }
}
