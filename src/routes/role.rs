use crate::controllers::role::*;
use axum::{
    routing::{delete, get, post},
    Router,
};

pub fn role_group_router() -> Router {
    Router::new()
        .route("/", get(role_group_list).post(role_group_create))
        .route("/:id", get(role_group_detail).delete(role_group_delete))
        .route("/:id/role", post(role_to_group))
        .route("/:id/role/:cid", delete(role_leave_group))
}

pub fn role_router() -> Router {
    Router::new()
        .route("/", get(role_list).post(role_create))
        .route("/:id", get(role_detail).delete(role_delete))
        .route("/:id/user", post(role_to_user))
        .route("/:id/user/:cid", delete(role_leave_user))
}
