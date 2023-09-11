use crate::controllers::dept::*;
use axum::{routing::get, Router};

pub fn dept_router() -> Router {
    Router::new()
        .route("/", get(dept_list))
        .route("/:id", get(dept_detail))
}
