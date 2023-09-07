mod user;
use axum::{routing::get, Router};
pub use user::user_router;

pub fn root_router() -> Router {
    Router::new().route("/", get(|| async { "Hello, world!" }))
}
