use axum::{extract::Path, http::StatusCode, Json};

pub async fn right_group_list() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "msg": "Right group list",
        })),
    )
}
pub async fn right_group_create() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::CREATED,
        Json(serde_json::json!({
            "msg": "Right group created",
        })),
    )
}
pub async fn right_group_detail(Path(id): Path<usize>) -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "msg": format!("Right group {}", id),
        })),
    )
}

pub async fn right_group_delete(Path(id): Path<usize>) -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "msg": format!("Right group {} deleted", id),
        })),
    )
}

pub async fn right_to_group(
    Path((id, cid)): Path<(usize, usize)>,
) -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "msg": format!("Right {} added to group {}", cid, id),
        })),
    )
}

pub async fn right_leave_group(
    Path((id, cid)): Path<(usize, usize)>,
) -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "msg": format!("Right {} removed from group {}", cid, id),
        })),
    )
}

pub async fn right_list() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "msg": "Right list",
        })),
    )
}

pub async fn right_create() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::CREATED,
        Json(serde_json::json!({
            "msg": "Right created",
        })),
    )
}

pub async fn right_detail(Path(id): Path<usize>) -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "msg": format!("Right {}", id),
        })),
    )
}

pub async fn right_delete(Path(id): Path<usize>) -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "msg": format!("Right {} deleted", id),
        })),
    )
}

pub async fn right_to_role(
    Path((id, cid)): Path<(usize, usize)>,
) -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "msg": format!("Right {} added to role {}", cid, id),
        })),
    )
}

pub async fn right_leave_role(
    Path((id, cid)): Path<(usize, usize)>,
) -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "msg": format!("Right {} removed from role {}", cid, id),
        })),
    )
}
