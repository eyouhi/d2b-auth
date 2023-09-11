mod dept;
mod right;
mod role;
mod root_test;
mod user;
use axum::{routing::get, Router};
use dept::*;
use right::*;
use role::*;
use user::*;

fn root_router() -> Router {
    Router::new().route("/", get(|| async { "Hello, world!" }))
}

pub fn route_app() -> Router {
    Router::new()
        .nest("/", root_router())
        .nest("/user", user_router())
        .nest("/user-group", user_group_router())
        .nest("/role", role_router())
        .nest("/role-group", role_group_router())
        .nest("/right", right_router())
        .nest("/right-group", right_group_router())
        .nest("/dept", dept_router())
}
