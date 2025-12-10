//! 认证相关测试
//!
//! 测试用户登录、注册、刷新令牌等功能

use axum::{
    body::Body,
    http::{self, Request, StatusCode},
    Router,
};
use jz::{
    models::auth::{LoginRequest, RegisterRequest, RefreshTokenRequest, LogoutRequest},
    services::mock_user_service::MockUserService,
    utils::jwt::Claims,
    models::user::User,
};
use serde_json::json;
use sqlx::types::chrono::NaiveDateTime;
use tower::ServiceExt; // for `call`, `oneshot`, and `ready`

// JWT 令牌生成测试
#[tokio::test]
async fn test_jwt_generation() {
    let user = User {
        user_id: 1,
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        phone: "1234567890".to_string(),
        password_hash: "hash".to_string(),
        user_type: "customer".to_string(),
        avatar_url: None,
        real_name: None,
        is_verified: false,
        balance: 0.0,
        status: "active".to_string(),
        created_at: NaiveDateTime::from_timestamp_opt(1609459200, 0).unwrap(),
        updated_at: NaiveDateTime::from_timestamp_opt(1609459200, 0).unwrap(),
    };
    
    let claims = Claims::new(&user);
    let secret = "test_secret";
    
    let token = claims.generate_token(secret).expect("Failed to generate token");
    assert!(!token.is_empty());
}

// JWT 令牌验证测试
#[tokio::test]
async fn test_jwt_validation() {
    let user = User {
        user_id: 1,
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        phone: "1234567890".to_string(),
        password_hash: "hash".to_string(),
        user_type: "customer".to_string(),
        avatar_url: None,
        real_name: None,
        is_verified: false,
        balance: 0.0,
        status: "active".to_string(),
        created_at: NaiveDateTime::from_timestamp_opt(1609459200, 0).unwrap(),
        updated_at: NaiveDateTime::from_timestamp_opt(1609459200, 0).unwrap(),
    };
    
    let claims = Claims::new(&user);
    let secret = "test_secret";
    
    let token = claims.generate_token(secret).expect("Failed to generate token");
    let validated_claims = Claims::validate_token(&token, secret).expect("Failed to validate token");
    
    assert_eq!(validated_claims.user_id, 1);
    assert_eq!(validated_claims.username, "testuser");
    assert_eq!(validated_claims.user_type, "customer");
}

// 模拟用户登录成功测试
#[tokio::test]
async fn test_mock_user_login_success() {
    let mock_service = MockUserService::new();
    
    let login_request = LoginRequest {
        identifier: "testuser".to_string(),
        password: "password123".to_string(),
    };
    
    let result = mock_service.login(&login_request).await;
    assert!(result.is_ok());
    
    let user = result.unwrap();
    assert_eq!(user.username, "testuser");
    assert_eq!(user.email, "test@example.com");
}

// 模拟用户登录失败测试
#[tokio::test]
async fn test_mock_user_login_failure() {
    let mock_service = MockUserService::new();
    
    let login_request = LoginRequest {
        identifier: "wronguser".to_string(),
        password: "wrongpass".to_string(),
    };
    
    let result = mock_service.login(&login_request).await;
    assert!(result.is_err());
}

// 模拟用户注册测试
#[tokio::test]
async fn test_mock_user_register() {
    let mock_service = MockUserService::new();
    
    let register_request = RegisterRequest {
        username: "newuser".to_string(),
        email: "newuser@example.com".to_string(),
        phone: "13900139000".to_string(),
        password: "password123".to_string(),
    };
    
    let result = mock_service.register(&register_request).await;
    assert!(result.is_ok());
    
    let user = result.unwrap();
    assert_eq!(user.username, "newuser");
    assert_eq!(user.email, "newuser@example.com");
}