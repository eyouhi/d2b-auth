use axum::{
    routing::{delete, get, post},
    Router,
};

use crate::controllers::user::*;

pub fn user_router() -> Router {
    Router::new()
        .route("/", get(user_list).post(user_create))
        .route("/:id", get(user_detail).delete(user_delete))
}

pub fn user_group_router() -> Router {
    Router::new()
        .route("/", get(user_group_list).post(user_group_create))
        .route("/:id", get(user_group_detail).delete(user_group_delete))
        .route("/:id/user", post(user_to_group))
        .route("/:id/user/:cid", delete(user_leave_group))
}
