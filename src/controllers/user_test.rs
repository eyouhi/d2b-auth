#[tokio::test]
async fn user_list() {
    use super::user::user_list;
    use axum::http::StatusCode;
    let (code, res) = user_list().await;
    assert_eq!(code, StatusCode::OK);
    assert!(res.0.is_array());
}

#[tokio::test]
async fn user_create() {
    use super::user::user_create;
    use axum::http::StatusCode;
    let (code, res) = user_create().await;
    assert_eq!(code, StatusCode::CREATED);
    assert_eq!(res.0.as_object().unwrap()["id"], 3);
}

#[tokio::test]
async fn user_detail() {
    use super::user::user_detail;
    use axum::http::StatusCode;
    let (code, res) = user_detail(axum::extract::Path(1)).await;
    assert_eq!(code, StatusCode::OK);
}

#[tokio::test]
async fn user_delete() {
    use super::user::user_delete;
    use axum::http::StatusCode;
    let (code, res) = user_delete(axum::extract::Path(1)).await;
    assert_eq!(code, StatusCode::OK);
    assert_eq!(res.0.as_object().unwrap()["msg"], "User 1 deleted");
}

#[tokio::test]
async fn user_group_list() {
    use super::user::user_group_list;
    use axum::http::StatusCode;
    let (code, res) = user_group_list().await;
    assert_eq!(code, StatusCode::OK);
    assert!(res.0.is_array());
}

#[tokio::test]
async fn user_group_create() {
    use super::user::user_group_create;
    use axum::http::StatusCode;
    let (code, res) = user_group_create().await;
    assert_eq!(code, StatusCode::CREATED);
    assert_eq!(res.0.as_object().unwrap()["id"], 3);
}

#[tokio::test]
async fn user_group_detail() {
    use super::user::user_group_detail;
    use axum::http::StatusCode;
    let (code, res) = user_group_detail(axum::extract::Path(1)).await;
    assert_eq!(code, StatusCode::OK);
}

#[tokio::test]
async fn user_group_delete() {
    use super::user::user_group_delete;
    use axum::http::StatusCode;
    let (code, res) = user_group_delete(axum::extract::Path(1)).await;
    assert_eq!(code, StatusCode::OK);
    assert_eq!(res.0.as_object().unwrap()["msg"], "User group 1 deleted");
}
