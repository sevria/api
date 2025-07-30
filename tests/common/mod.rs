use std::sync::Arc;

use anyhow::Result;
use axum_test::TestServer;
use dotenvy::dotenv;
use envconfig::Envconfig;
use sevria_api::{config::Config, http};
use sqlx::postgres::PgPoolOptions;

pub async fn setup() -> Result<TestServer> {
    dotenv().ok();

    let config = Arc::new(Config::init_from_env()?);
    let db = Arc::new(PgPoolOptions::new().connect(&config.database_url).await?);
    let router = http::new_router(config, db);
    let server = TestServer::new(router)?;

    Ok(server)
}
