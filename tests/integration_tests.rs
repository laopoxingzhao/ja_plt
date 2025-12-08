//! 集成测试示例
//! 
//! 这些测试需要运行中的数据库实例

use axum::{
    body::Body,
    http::{self, Request, StatusCode},
    routing::{get, post},
    Router,
};
use jz::database;
use serde_json::json;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower::ServiceExt; // for `call`, `oneshot`, and `ready`

/*
#[tokio::test]
async fn test_user_registration_integration() {
    // 构建应用
    let pool = database::init_db_pool().await.expect("Failed to initialize database pool");
    let app = Router::new()
        .nest("/users", jz::handler::user_routes())
        .with_state(pool);

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
    let pool = database::init_db_pool().await.expect("Failed to initialize database pool");
    let app = Router::new()
        .nest("/users", jz::handler::user_routes())
        .with_state(pool);

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
*/