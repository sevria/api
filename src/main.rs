use anyhow::Result;
use dotenvy::dotenv;
use envconfig::Envconfig;
use sevria_api::{config::Config, http};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let config = Config::init_from_env()?;
    let router = http::new_router();
    let listener = tokio::net::TcpListener::bind(&config.http_address).await?;

    println!("running http server on {}", listener.local_addr()?);

    axum::serve(listener, router).await?;

    Ok(())
}
