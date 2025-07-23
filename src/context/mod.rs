use sqlx::{Pool, Postgres};

pub struct Context {
    pub db: Pool<Postgres>,
}
