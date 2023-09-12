#[tokio::test]
async fn dept_list() {
    use super::dept::dept_list;
    use axum::http::StatusCode;
    let (code, res) = dept_list().await;
    assert_eq!(code, StatusCode::OK);
    assert!(res.0.is_object());
}

#[tokio::test]
async fn dept_detail() {
    use super::dept::dept_detail;
    use axum::http::StatusCode;
    let (code, _) = dept_detail(axum::extract::Path(1)).await;
    assert_eq!(code, StatusCode::OK);
}
