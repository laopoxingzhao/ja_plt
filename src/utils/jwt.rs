use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey, errors::{Error, ErrorKind}};
use crate::models::user::User;
use std::collections::HashMap;
use uuid::Uuid;
use once_cell::sync::Lazy;
use tokio::sync::RwLock;
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize,Clone)]
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
    // Token类型(access或refresh)
    pub token_type: String,
}

impl Claims {
    // 创建新的Claims实例
    pub fn new(user: &User, token_type: &str, expiration_seconds: usize) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs() as usize;

        Self {
            // 根据token类型设置aud字段
            aud: match token_type {
                "access" => "access-token-audience".to_string(),
                "refresh" => "refresh-token-audience".to_string(),
                _ => "default-audience".to_string(),
            },
            exp: now + expiration_seconds, // 设置过期时间为当前时间加上指定秒数
            iat: now, // 设置签发时间为当前时间
            iss: "jz-service".to_string(), // 设置签发者
            nbf: now, // 设置not before时间为当前时间
            sub: user.user_id.to_string(), // 设置subject为用户ID
            user_id: user.user_id,
            username: user.username.clone(),
            user_type: user.user_type.clone(),
            token_type: token_type.to_string(),
        }
    }

    // 生成JWT token
    pub fn generate_token(&self, secret: &str) -> Result<String, Error> {
        let encoding_key = EncodingKey::from_secret(secret.as_ref());
        encode(&Header::default(), self, &encoding_key)
    }

    // 验证JWT token
    pub fn validate_token(
        token: &str,
        secret: &str,
        expected_type: Option<&str>,
    ) -> Result<Claims, Error> {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_audience(&["access-token-audience", "refresh-token-audience"]); // 设置期望的audience
        validation.set_issuer(&["jz-service"]); // 设置期望的issuer
        
        // 如果指定了token类型，则验证
        if expected_type.is_some() {
            validation.validate_aud = false; // 禁用aud验证以便自定义验证
        }

        let decoding_key = DecodingKey::from_secret(secret.as_ref());
        let decoded = decode::<Claims>(token, &decoding_key, &validation)?;
        
        // 验证token类型
        if let Some(token_type) = expected_type {
            if decoded.claims.token_type != token_type {
                return Err(Error::from(ErrorKind::InvalidToken));
            }
        }
        
        Ok(decoded.claims)
    }
}

// 存储刷新令牌的简单内存存储
// 在生产环境中，应该使用Redis或其他持久化存储
type RefreshTokenStore = HashMap<String, (i32, String)>;
static REFRESH_TOKENS: Lazy<Arc<RwLock<RefreshTokenStore>>> = Lazy::new(|| {
    Arc::new(RwLock::new(HashMap::<String, (i32, String)>::new()))
});

// 保存刷新令牌
pub async fn store_refresh_token(token: String, user_id: i32, username: String) {
    let mut tokens = REFRESH_TOKENS.write().await;
    tokens.insert(token, (user_id, username));
}

// 验证刷新令牌
pub async fn validate_refresh_token(token: &str) -> Option<(i32, String)> {
    let tokens = REFRESH_TOKENS.read().await;
    tokens.get(token).cloned()
}

// 移除刷新令牌(登出时使用)
pub async fn remove_refresh_token(token: &str) {
    let mut tokens = REFRESH_TOKENS.write().await;
    tokens.remove(token);
}

// 生成刷新令牌
pub fn generate_refresh_token() -> String {
    Uuid::new_v4().to_string()
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
        
        let claims = Claims::new(&user, "access", 3600);
        let secret = "test_secret";
        
        // 测试生成token
        let token = claims.generate_token(secret).expect("Failed to generate token");
        assert!(!token.is_empty());
        
        // 测试验证token
        let validated_claims = Claims::validate_token(&token, secret, Some("access")).expect("Failed to validate token");
        assert_eq!(validated_claims.user_id, 1);
        assert_eq!(validated_claims.username, "testuser");
        assert_eq!(validated_claims.user_type, "customer");
        assert_eq!(validated_claims.token_type, "access");
    }
}