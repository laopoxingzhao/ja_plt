pub mod user_service;
pub mod service_service;
pub mod order_service;

// Mock服务用于测试
#[cfg(test)]
pub mod mock_user_service {
    use crate::models::{
        user::User,
        auth::{LoginRequest, RegisterRequest}
    };
    use sqlx::types::chrono::NaiveDateTime;

    pub struct MockUserService;

    impl MockUserService {
        pub fn new() -> Self {
            Self
        }

        pub async fn login(&self, payload: &LoginRequest) -> Result<User, Box<dyn std::error::Error>> {
            // 模拟登录逻辑
            if payload.identifier == "testuser" && payload.password == "password123" {
                Ok(User {
                    user_id: 1,
                    username: "testuser".to_string(),
                    password_hash: "$argon2id$v=19$m=4096,t=3,p=1$c29tZXNhbHQ$hashedpassword".to_string(),
                    email: "test@example.com".to_string(),
                    phone: "13800138000".to_string(),
                    user_type: "customer".to_string(),
                    avatar_url: Some("https://example.com/avatar.jpg".to_string()),
                    real_name: Some("张三".to_string()),
                    is_verified: true,
                    balance: 100.0,
                    status: "active".to_string(),
                    created_at: NaiveDateTime::from_timestamp_opt(1609459200, 0).unwrap(),
                    updated_at: NaiveDateTime::from_timestamp_opt(1609459200, 0).unwrap(),
                })
            } else {
                Err("用户名或密码错误".into())
            }
        }

        pub async fn register(&self, _payload: &RegisterRequest) -> Result<User, Box<dyn std::error::Error>> {
            // 模拟注册逻辑
            Ok(User {
                user_id: 2,
                username: "newuser".to_string(),
                password_hash: "$argon2id$v=19$m=4096,t=3,p=1$c29tZXNhbHQ$hashedpassword".to_string(),
                email: "newuser@example.com".to_string(),
                phone: "13900139000".to_string(),
                user_type: "customer".to_string(),
                avatar_url: None,
                real_name: None,
                is_verified: false,
                balance: 0.0,
                status: "active".to_string(),
                created_at: NaiveDateTime::from_timestamp_opt(1609459200, 0).unwrap(),
                updated_at: NaiveDateTime::from_timestamp_opt(1609459200, 0).unwrap(),
            })
        }
    }
}