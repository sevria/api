use std::sync::Arc;

use async_trait::async_trait;
use sqlx::{Pool, Postgres, QueryBuilder};

use crate::util::error::Error;

use super::{
    model::{GetUserRequest, User},
    repository::UserRepository,
};

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
    async fn create(&self, data: &User) -> Result<User, Error> {
        let mut query = QueryBuilder::new("INSERT INTO users (name, email, password) VALUES (");

        query.push_bind(&data.name);
        query.push(", ");
        query.push_bind(&data.email);
        query.push(", ");
        query.push_bind(&data.password);
        query.push(") RETURNING *");

        match query.build_query_as::<User>().fetch_one(&*self.db).await {
            Ok(user) => Ok(user),
            Err(err) => {
                log::error!("failed to create user: {}", err);
                Err(Error::Internal)
            }
        }
    }

    async fn get(&self, req: &GetUserRequest) -> Result<User, Error> {
        let mut query = QueryBuilder::new("SELECT * FROM users WHERE");

        match req {
            GetUserRequest::Id(id) => {
                query.push(" id = ");
                query.push_bind(id);
            }
            GetUserRequest::Email(email) => {
                query.push(" email = ");
                query.push_bind(email);
            }
        }

        match query.build_query_as::<User>().fetch_one(&*self.db).await {
            Ok(user) => Ok(user),
            Err(sqlx::Error::RowNotFound) => Err(Error::NotFound),
            Err(err) => {
                log::error!("failed to get user: {}", err);
                return Err(Error::Internal);
            }
        }
    }
}
