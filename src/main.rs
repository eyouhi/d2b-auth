use anyhow::{Ok, Result};
use axum::{routing::get, Router};
mod proj_env;
use std::env;

const DEFAULT_HOSTADDR: &str = "0.0.0.0:3000";

#[tokio::main]
async fn main() -> Result<()> {
    proj_env::load().unwrap_or_else(|err| println!("Error loading .env file: {}", err));
    let app = Router::new().route("/", get(|| async { "Hello, world!" }));
    let host = env::var("HOSTADDR").unwrap_or(DEFAULT_HOSTADDR.to_string());
    println!("Listening on {}", host);
    axum::Server::bind(&host.parse()?)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
