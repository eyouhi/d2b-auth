use axum::{extract::Path, http::StatusCode, Json};

use crate::modules::dept::Dept;

pub async fn dept_list() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "msg": "Dept list",
        })),
    )
}

pub async fn dept_detail(Path(id): Path<usize>) -> (StatusCode, Json<Dept>) {
    let mut dept = Dept::default();
    dept.set_id(id);
    (StatusCode::OK, Json(dept))
}
