mod _test;
mod role;
mod user;
use axum::{routing::get, Router};
pub use role::*;
pub use user::*;

pub fn root_router() -> Router {
    Router::new().route("/", get(|| async { "Hello, world!" }))
}
