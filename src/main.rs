use anyhow::Result;
use axum::{Router, response::Html, routing::get};

#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new().route("/", get(handler));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await?;

    println!("listening on {}", listener.local_addr()?);

    axum::serve(listener, app).await?;

    Ok(())
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello from Sevria API!</h1>")
}
