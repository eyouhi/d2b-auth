#[tokio::test]
async fn user_list() {
    use super::user_list;
    use axum::http::StatusCode;
    let (code, res) = user_list().await;
    assert_eq!(code, StatusCode::OK);
    assert!(res.0.is_array());
}

#[tokio::test]
async fn user_create() {
    use super::user_create;
    use axum::http::StatusCode;
    let (code, res) = user_create().await;
    assert_eq!(code, StatusCode::CREATED);
    assert_eq!(res.0.as_object().unwrap()["id"], 3);
}

#[tokio::test]
async fn user_detail() {
    use super::user_detail;
    use axum::http::StatusCode;
    let (code, res) = user_detail(axum::extract::Path(1)).await;
    assert_eq!(code, StatusCode::OK);
    assert_eq!(res.0.as_object().unwrap()["name"], "Alice");
    let (code, res) = user_detail(axum::extract::Path(2)).await;
    assert_eq!(code, StatusCode::OK);
    assert_eq!(res.0.as_object().unwrap()["name"], "Bob");
}

#[tokio::test]
async fn user_delete() {
    use super::user_delete;
    use axum::http::StatusCode;
    let (code, res) = user_delete(axum::extract::Path(1)).await;
    assert_eq!(code, StatusCode::OK);
    assert_eq!(res.0.as_object().unwrap()["msg"], "User 1 deleted");
}

#[tokio::test]
async fn user_group_list() {
    use super::user_group_list;
    use axum::http::StatusCode;
    let (code, res) = user_group_list().await;
    assert_eq!(code, StatusCode::OK);
    assert!(res.0.is_array());
}

#[tokio::test]
async fn user_group_create() {
    use super::user_group_create;
    use axum::http::StatusCode;
    let (code, res) = user_group_create().await;
    assert_eq!(code, StatusCode::CREATED);
    assert_eq!(res.0.as_object().unwrap()["id"], 3);
}

#[tokio::test]
async fn user_group_detail() {
    use super::user_group_detail;
    use axum::http::StatusCode;
    let (code, res) = user_group_detail(axum::extract::Path(1)).await;
    assert_eq!(code, StatusCode::OK);
    assert_eq!(res.0.as_object().unwrap()["name"], "钉钉项目组");
}

#[tokio::test]
async fn user_group_delete() {
    use super::user_group_delete;
    use axum::http::StatusCode;
    let (code, res) = user_group_delete(axum::extract::Path(1)).await;
    assert_eq!(code, StatusCode::OK);
    assert_eq!(res.0.as_object().unwrap()["msg"], "User group 1 deleted");
}
