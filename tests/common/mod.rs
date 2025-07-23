use std::sync::Arc;

use anyhow::Result;
use axum_test::TestServer;
use dotenvy::dotenv;
use envconfig::Envconfig;
use sevria_api::{config::Config, context::Context, http};
use sqlx::postgres::PgPoolOptions;

#[allow(dead_code)]
pub struct TestState {
    pub ctx: Arc<Context>,
    pub server: TestServer,
}

pub async fn setup() -> Result<TestState> {
    dotenv().ok();

    let config = Config::init_from_env()?;
    let db = PgPoolOptions::new().connect(&config.database_url).await?;

    let ctx = Arc::new(Context { db });
    let router = http::new_router(ctx.clone());
    let server = TestServer::new(router)?;

    Ok(TestState { ctx, server })
}
