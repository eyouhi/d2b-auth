use axum::{extract::Path, http::StatusCode, Json};

use crate::modules::role::{Role, RoleGroup};

pub async fn role_leave_group(
    Path((id, cid)): Path<(usize, usize)>,
) -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "msg": format!("Role {} removed from group {}", cid, id),
        })),
    )
}

pub async fn role_group_create() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::CREATED,
        Json(serde_json::json!({
            "msg": "Role group created",
        })),
    )
}

pub async fn role_group_list() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "msg": "Role group list",
        })),
    )
}

pub async fn role_group_detail(Path(id): Path<usize>) -> (StatusCode, Json<RoleGroup>) {
    let mut role_group = RoleGroup::default();
    role_group.set_id(id);
    (StatusCode::OK, Json(role_group))
}

pub async fn role_group_delete(Path(id): Path<usize>) -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "msg": format!("Role group {} deleted", id),
        })),
    )
}

pub async fn role_to_group(
    Path((id, cid)): Path<(usize, usize)>,
) -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "msg": format!("Role {} added to group {}", cid, id),
        })),
    )
}

pub async fn role_list() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "msg": "Role list",
        })),
    )
}

pub async fn role_create() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::CREATED,
        Json(serde_json::json!({
            "msg": "Role created",
        })),
    )
}

pub async fn role_detail(Path(id): Path<usize>) -> (StatusCode, Json<Role>) {
    let mut role = Role::default();
    role.set_id(id);
    (StatusCode::OK, Json(role))
}

pub async fn role_delete(Path(id): Path<usize>) -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "msg": format!("Role {} deleted", id),
        })),
    )
}

pub async fn role_to_user(
    Path((id, cid)): Path<(usize, usize)>,
) -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "msg": format!("Role {} added to user {}", cid, id),
        })),
    )
}

pub async fn role_leave_user(
    Path((id, cid)): Path<(usize, usize)>,
) -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "msg": format!("Role {} removed from user {}", cid, id),
        })),
    )
}
