
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey, errors::Error};
use crate::models::user::User;

#[derive(Debug, Serialize, Deserialize,Default)]
pub struct Claims {
    aud: String, // Optional. Audience 接收方
    exp: usize, // Required  过期时间 (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    iat: usize, // Optional.  签发时间 Issued at (as UTC timestamp)
    iss: String, // Optional.   签发者 Issuer
    nbf: usize, // Optional. 生效时间 Not Before (as UTC timestamp)
    sub: String, // Optional. 主题 Subject (whom token refers to)
    // 添加用户相关信息
    pub user_id: i32,
    pub username: String,
    pub user_type: String,
}

impl Claims {
    // 创建新的Claims实例
    pub fn new(user: &User) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs() as usize;

        Self {
            sub: user.user_id.to_string(),
            iss: "jz-service".to_string(),
            aud: "jz-client".to_string(),
            iat: now,
            exp: now + 3600 * 24, // 24小时后过期
            nbf: now,
            user_id: user.user_id,
            username: user.username.clone(),
            user_type: user.user_type.clone(),
        }
    }

    // 生成JWT token
    pub fn generate_token(&self, secret: &str) -> Result<String, Error> {
        let header = Header::new(Algorithm::HS256);
        let encoding_key = EncodingKey::from_secret(secret.as_ref());
        encode(&header, self, &encoding_key)
    }

    // 验证JWT token
    pub fn validate_token(
        token: &str,
        secret: &str,
        
    ) -> Result<Claims, Error> {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_audience(&["jz-client"]); // 设置期望的audience
        validation.set_issuer(&["jz-service"]); // 设置期望的issuer

        let decoding_key = DecodingKey::from_secret(secret.as_ref());
        let decoded = decode::<Claims>(token, &decoding_key, &validation)?;
        Ok(decoded.claims)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::types::chrono::NaiveDateTime;

    #[test]
    fn test_generate_and_validate_token() {
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
        
        // 测试生成token
        let token = claims.generate_token(secret).expect("Failed to generate token");
        assert!(!token.is_empty());
        
        // 测试验证token
        let validated_claims = Claims::validate_token(&token, secret).expect("Failed to validate token");
        assert_eq!(validated_claims.user_id, 1);
        assert_eq!(validated_claims.username, "testuser");
        assert_eq!(validated_claims.user_type, "customer");
    }
}