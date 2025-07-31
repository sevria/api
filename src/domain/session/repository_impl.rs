use std::sync::Arc;

use async_trait::async_trait;
use sqlx::{Pool, Postgres, QueryBuilder};

use crate::{
    domain::session::{model::Session, repository::SessionRepository},
    util::error::Error,
};

pub struct SessionRepositoryImpl {
    db: Arc<Pool<Postgres>>,
}

impl SessionRepositoryImpl {
    pub fn new(db: Arc<Pool<Postgres>>) -> SessionRepositoryImpl {
        SessionRepositoryImpl { db }
    }
}

#[async_trait]
impl SessionRepository for SessionRepositoryImpl {
    async fn create(&self, data: &Session) -> Result<Session, Error> {
        let mut query =
            QueryBuilder::new("INSERT INTO sessions (id, token, user_id, expires_at) VALUES (");

        query.push_bind(&data.id);
        query.push(", ");
        query.push_bind(&data.token);
        query.push(", ");
        query.push_bind(&data.user_id);
        query.push(", ");
        query.push_bind(&data.expires_at);
        query.push(") RETURNING *");

        match query.build_query_as::<Session>().fetch_one(&*self.db).await {
            Ok(session) => Ok(session),
            Err(err) => {
                log::error!("failed to create session: {}", err);
                Err(Error::Internal)
            }
        }
    }

    async fn get(&self, token: &str, user_id: &str) -> Result<Session, Error> {
        let mut query = QueryBuilder::new("SELECT * FROM sessions WHERE ");

        query.push("token = ");
        query.push_bind(&token);
        query.push(" AND user_id = ");
        query.push_bind(&user_id);

        match query.build_query_as::<Session>().fetch_one(&*self.db).await {
            Ok(session) => Ok(session),
            Err(err) => {
                log::error!("failed to get session: {}", err);
                Err(Error::Internal)
            }
        }
    }
}
