use std::sync::Arc;

use anyhow::Result;
use dotenvy::dotenv;
use env_logger::Env;
use envconfig::Envconfig;
use sevria_api::{
    config::Config,
    context::Context,
    domain::user::model::{User, UserStatus},
    http,
};
use sqlx::{migrate, postgres::PgPoolOptions};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let config = Arc::new(Config::init_from_env()?);
    let db = Arc::new(PgPoolOptions::new().connect(&config.database_url).await?);

    // Run database migrations
    migrate!().run(&*db).await?;

    // Create context containing repositories and services
    let context = Context::new(config.clone(), db);

    // Create default admin user if no users exist
    let total_users = context.user_repository.count().await?;
    if total_users == 0 {
        let user = User::new(
            String::from("Administrator"),
            config.default_admin_email.clone(),
            config.default_admin_password.clone(),
            UserStatus::Active,
        )?;
        match context.user_repository.create(&user).await {
            Ok(_) => log::info!("Created default admin user"),
            Err(err) => log::error!("Failed to create default admin user: {}", err),
        };
    }

    let router = http::new_router(config.clone(), context);
    let listener = tokio::net::TcpListener::bind(&config.http_address).await?;

    log::info!("Running HTTP server on {}", listener.local_addr()?);

    axum::serve(listener, router).await?;

    Ok(())
}
