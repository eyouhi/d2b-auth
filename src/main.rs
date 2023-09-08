mod controllers;
mod proj_env;
mod repo;
mod routes;

use anyhow::{Ok, Result};
use axum::Router;
use std::env;

const DEFAULT_HOSTADDR: &str = "0.0.0.0:3000";

#[tokio::main]
async fn main() -> Result<()> {
    proj_env::load().unwrap_or_else(|err| println!("Error loading .env file: {}", err));
    let app = Router::new()
        .nest("/", routes::root_router())
        .nest("/user", routes::user_router())
        .nest("/user-group", routes::user_group_router());
    let host = env::var("D2B_AUTH_ADDR").unwrap_or(DEFAULT_HOSTADDR.to_string());
    println!("Listening on {}", host);
    axum::Server::bind(&host.parse()?)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
