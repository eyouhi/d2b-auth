use axum::{extract::Path, http::StatusCode, Json};

pub async fn dept_list() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "msg": "Dept list",
        })),
    )
}

pub async fn dept_detail(Path(id): Path<usize>) -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "msg": format!("Dept {}", id),
        })),
    )
}
