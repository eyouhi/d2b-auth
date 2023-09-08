use axum::{
    extract::{Json, Path},
    http::StatusCode,
};

pub async fn user_list() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::OK,
        Json(serde_json::json!([{
            "id": 1,
            "name": "Alice",
        }, {
            "id": 2,
            "name": "Bob",
        }])),
    )
}

pub async fn user_create() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::CREATED,
        Json(serde_json::json!({
            "id": 3
        })),
    )
}

pub async fn user_detail(Path(id): Path<usize>) -> (StatusCode, Json<serde_json::Value>) {
    match id {
        1 => (
            StatusCode::OK,
            Json(serde_json::json!({
                "id": 1,
                "name": "Alice",
            })),
        ),
        2 => (
            StatusCode::OK,
            Json(serde_json::json!({
                "id": 2,
                "name": "Bob",
            })),
        ),
        _ => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": "User not found",
            })),
        ),
    }
}

pub async fn user_delete(Path(id): Path<usize>) -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "msg": format!("User {} deleted", id),
        })),
    )
}

pub async fn user_group_list() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::OK,
        Json(serde_json::json!([{
            "id": 1,
            "name": "钉钉项目组",
        }, {
            "id": 2,
            "name": "飞书项目组",
        }])),
    )
}

pub async fn user_group_create() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::CREATED,
        Json(serde_json::json!({
            "id": 3
        })),
    )
}

pub async fn user_group_detail(Path(id): Path<usize>) -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "id": id,
            "name": "钉钉项目组",
            "members": [{
                "id": 1,
                "name": "Alice",
            }, {
                "id": 2,
                "name": "Bob",
            }],
        })),
    )
}

pub async fn user_group_delete(Path(id): Path<usize>) -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "msg": format!("User group {} deleted", id),
        })),
    )
}

pub async fn user_to_group(Path(id): Path<usize>) -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "msg": format!("User 1,2 added to group {}", id),
        })),
    )
}

pub async fn user_leave_group(
    Path((id, cid)): Path<(usize, usize)>,
) -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "msg": format!("User {} removed from group {}",cid, id),
        })),
    )
}
