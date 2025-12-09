use jz::utils::jwt::Claims;
use jz::models::user::User;
use sqlx::types::chrono::NaiveDateTime;

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