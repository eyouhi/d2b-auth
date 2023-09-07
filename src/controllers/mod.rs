use axum::{http::StatusCode, Json};
mod _test;

pub async fn user_list() -> (StatusCode, Json<serde_json::Value>) {
    todo!()
}

pub async fn user_detail() -> (StatusCode, Json<serde_json::Value>) {
    todo!()
}
