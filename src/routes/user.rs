use axum::{routing::get, Router};

use crate::controllers::{user_detail, user_list};

pub fn user_router() -> Router {
    Router::new()
        .route("/", get(user_list))
        .route("/:id", get(user_detail))
}
