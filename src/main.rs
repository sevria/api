use std::sync::Arc;

use anyhow::Result;
use dotenvy::dotenv;
use env_logger::Env;
use envconfig::Envconfig;
use sevria_api::{config::Config, context::Context, http};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let config = Config::init_from_env()?;
    let db = PgPoolOptions::new().connect(&config.database_url).await?;

    let ctx = Arc::new(Context { db });
    let router = http::new_router(ctx);
    let listener = tokio::net::TcpListener::bind(&config.http_address).await?;

    println!("running http server on {}", listener.local_addr()?);

    axum::serve(listener, router).await?;

    Ok(())
}
