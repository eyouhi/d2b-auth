#[tokio::test]
async fn role_list() {
    use super::role::role_list;
    use axum::http::StatusCode;
    let (code, res) = role_list().await;
    assert_eq!(code, StatusCode::OK);
    assert!(res.0.is_object());
}

#[tokio::test]
async fn role_create() {
    use super::role::role_create;
    use axum::http::StatusCode;
    let (code, res) = role_create().await;
    assert_eq!(code, StatusCode::CREATED);
    assert!(res.0.is_object());
}

#[tokio::test]
async fn role_detail() {
    use super::role::role_detail;
    use axum::http::StatusCode;
    let (code, res) = role_detail(axum::extract::Path(1)).await;
    assert_eq!(code, StatusCode::OK);
    assert!(res.0.is_object());
}

#[tokio::test]
async fn role_delete() {
    use super::role::role_delete;
    use axum::http::StatusCode;
    let (code, res) = role_delete(axum::extract::Path(1)).await;
    assert_eq!(code, StatusCode::OK);
    assert!(res.0.is_object());
}

#[tokio::test]
async fn role_group_list() {
    use super::role::role_group_list;
    use axum::http::StatusCode;
    let (code, res) = role_group_list().await;
    assert_eq!(code, StatusCode::OK);
    assert!(res.0.is_object());
}

#[tokio::test]
async fn role_group_create() {
    use super::role::role_group_create;
    use axum::http::StatusCode;
    let (code, res) = role_group_create().await;
    assert_eq!(code, StatusCode::CREATED);
    assert!(res.0.is_object());
}

#[tokio::test]
async fn role_group_detail() {
    use super::role::role_group_detail;
    use axum::http::StatusCode;
    let (code, res) = role_group_detail(axum::extract::Path(1)).await;
    assert_eq!(code, StatusCode::OK);
    assert!(res.0.is_object());
}

#[tokio::test]
async fn role_group_delete() {
    use super::role::role_group_delete;
    use axum::http::StatusCode;
    let (code, res) = role_group_delete(axum::extract::Path(1)).await;
    assert_eq!(code, StatusCode::OK);
    assert!(res.0.is_object());
}

#[tokio::test]
async fn role_to_group() {
    use super::role::role_to_group;
    use axum::http::StatusCode;
    let (code, res) = role_to_group(axum::extract::Path((1, 1))).await;
    assert_eq!(code, StatusCode::OK);
    assert!(res.0.is_object());
}

#[tokio::test]
async fn role_leave_group() {
    use super::role::role_leave_group;
    use axum::http::StatusCode;
    let (code, res) = role_leave_group(axum::extract::Path((1, 1))).await;
    assert_eq!(code, StatusCode::OK);
    assert!(res.0.is_object());
}

#[tokio::test]
async fn role_to_user() {
    use super::role::role_to_user;
    use axum::http::StatusCode;
    let (code, res) = role_to_user(axum::extract::Path((1, 1))).await;
    assert_eq!(code, StatusCode::OK);
    assert!(res.0.is_object());
}

#[tokio::test]
async fn role_leave_user() {
    use super::role::role_leave_user;
    use axum::http::StatusCode;
    let (code, res) = role_leave_user(axum::extract::Path((1, 1))).await;
    assert_eq!(code, StatusCode::OK);
    assert!(res.0.is_object());
}
