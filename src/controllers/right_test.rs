#[tokio::test]
async fn right_list() {
    use super::right::right_list;
    use axum::http::StatusCode;
    let (code, res) = right_list().await;
    assert_eq!(code, StatusCode::OK);
    assert!(res.0.is_object());
}

#[tokio::test]
async fn right_create() {
    use super::right::right_create;
    use axum::http::StatusCode;
    let (code, res) = right_create().await;
    assert_eq!(code, StatusCode::CREATED);
    assert!(res.0.is_object());
}

#[tokio::test]
async fn right_detail() {
    use super::right::right_detail;
    use axum::http::StatusCode;
    let (code, res) = right_detail(axum::extract::Path(1)).await;
    assert_eq!(code, StatusCode::OK);
    assert!(res.0.is_object());
}

#[tokio::test]
async fn right_delete() {
    use super::right::right_delete;
    use axum::http::StatusCode;
    let (code, res) = right_delete(axum::extract::Path(1)).await;
    assert_eq!(code, StatusCode::OK);
    assert!(res.0.is_object());
}

#[tokio::test]
async fn right_group_list() {
    use super::right::right_group_list;
    use axum::http::StatusCode;
    let (code, res) = right_group_list().await;
    assert_eq!(code, StatusCode::OK);
    assert!(res.0.is_object());
}

#[tokio::test]
async fn right_group_create() {
    use super::right::right_group_create;
    use axum::http::StatusCode;
    let (code, res) = right_group_create().await;
    assert_eq!(code, StatusCode::CREATED);
    assert!(res.0.is_object());
}

#[tokio::test]
async fn right_group_detail() {
    use super::right::right_group_detail;
    use axum::http::StatusCode;
    let (code, res) = right_group_detail(axum::extract::Path(1)).await;
    assert_eq!(code, StatusCode::OK);
    assert!(res.0.is_object());
}

#[tokio::test]
async fn right_group_delete() {
    use super::right::right_group_delete;
    use axum::http::StatusCode;
    let (code, res) = right_group_delete(axum::extract::Path(1)).await;
    assert_eq!(code, StatusCode::OK);
    assert!(res.0.is_object());
}

#[tokio::test]
async fn right_to_group() {
    use super::right::right_to_group;
    use axum::http::StatusCode;
    let (code, res) = right_to_group(axum::extract::Path((1, 1))).await;
    assert_eq!(code, StatusCode::OK);
    assert!(res.0.is_object());
}

#[tokio::test]
async fn right_leave_group() {
    use super::right::right_leave_group;
    use axum::http::StatusCode;
    let (code, res) = right_leave_group(axum::extract::Path((1, 1))).await;
    assert_eq!(code, StatusCode::OK);
    assert!(res.0.is_object());
}

#[tokio::test]
async fn right_to_role() {
    use super::right::right_to_role;
    use axum::http::StatusCode;
    let (code, res) = right_to_role(axum::extract::Path((1, 1))).await;
    assert_eq!(code, StatusCode::OK);
    assert!(res.0.is_object());
}

#[tokio::test]
async fn right_leave_role() {
    use super::right::right_leave_role;
    use axum::http::StatusCode;
    let (code, res) = right_leave_role(axum::extract::Path((1, 1))).await;
    assert_eq!(code, StatusCode::OK);
    assert!(res.0.is_object());
}
