mod controllers;
mod modules;
mod proj_env;
mod repo;
mod routes;
use anyhow::{Ok, Result};
use std::env;

use routes::route_app;

const DEFAULT_HOSTADDR: &str = "0.0.0.0:3000";

#[tokio::main]
async fn main() -> Result<()> {
    proj_env::load().unwrap_or_else(|err| println!("Error loading .env file: {}", err));
    let host = env::var("D2B_AUTH_ADDR").unwrap_or(DEFAULT_HOSTADDR.to_string());
    println!("Listening on {}", host);
    axum::Server::bind(&host.parse()?)
        .serve(route_app().into_make_service())
        .await?;
    Ok(())
}
