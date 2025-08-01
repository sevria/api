use std::sync::Arc;

use anyhow::Result;
use axum_test::TestServer;
use dotenvy::dotenv;
use envconfig::Envconfig;
use sevria_api::{config::Config, context::Context, http};
use sqlx::postgres::PgPoolOptions;

pub async fn setup() -> Result<TestServer> {
    dotenv().ok();

    let config = Arc::new(Config::init_from_env()?);
    let db = Arc::new(PgPoolOptions::new().connect(&config.database_url).await?);

    // Create context containing repositories and services
    let context = Context::new(config, db);

    let router = http::new_router(context);
    let server = TestServer::new(router)?;

    Ok(server)
}

#[allow(dead_code)]
pub struct TestCase {
    pub data: serde_json::Value,
    pub expected_error: String,
}
