use axum::{
    routing::{delete, get, post},
    Router,
};

use crate::controllers::right::*;

pub fn right_group_router() -> Router {
    Router::new()
        .route("/", get(right_group_list).post(right_group_create))
        .route("/:id", get(right_group_detail).delete(right_group_delete))
        .route("/:id/right", post(right_to_group))
        .route("/:id/right/:cid", delete(right_leave_group))
}

pub fn right_router() -> Router {
    Router::new()
        .route("/", get(right_list).post(right_create))
        .route("/:id", get(right_detail).delete(right_delete))
        .route("/:id/role", post(right_to_role))
        .route("/:id/role/:cid", delete(right_leave_role))
}
