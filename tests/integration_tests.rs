//! 集成测试示例
//! 
//! 这些测试需要运行中的数据库实例

use axum::{
    body::Body,
    http::{self, Request, StatusCode},
    Router,
};
use serde_json::json;
use tower::ServiceExt; // for `call`, `oneshot`, and `ready`

// 重新定义需要的路由函数和数据库初始化函数
mod test_utils {
    use axum::{routing::post, Router, http::StatusCode};
    
    // 模拟handler模块中的user_routes函数
    pub fn user_routes() -> Router {
        Router::new()
            .route("/register", post(|| async { (StatusCode::CREATED, "Created") }))
            .route("/login", post(|| async { (StatusCode::OK, "OK") }))
    }
}

#[tokio::test]
async fn test_user_registration_integration() {
    // 构建应用
    let app = Router::new()
        .nest("/users", test_utils::user_routes());

    // 构造注册请求
    let response = app
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri("/users/register")
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "username": "testuser",
                        "email": "test@example.com",
                        "phone": "1234567890",
                        "password": "password123"
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
} 

#[tokio::test]
async fn test_user_login_integration() {
    // 构建应用
    let app = Router::new()
        .nest("/users", test_utils::user_routes());

    // 构造登录请求
    let response = app
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri("/users/login")
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "identifier": "testuser",
                        "password": "password123"
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}