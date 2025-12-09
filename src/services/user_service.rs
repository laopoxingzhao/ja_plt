//! 用户业务逻辑层
//!
//! 处理用户相关的业务逻辑

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use sqlx::mysql::MySqlPool;
use std::fmt;

use crate::{
    models::{user::User, auth::{LoginRequest, RegisterRequest}},
    repositories::UserRepository,
};

#[derive(Debug)]
pub enum UserServiceError {
    DatabaseError(sqlx::Error),
    AuthenticationError(String),
    RegistrationError(String),
    PasswordHashError(argon2::password_hash::Error),
}

impl fmt::Display for UserServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UserServiceError::DatabaseError(e) => write!(f, "数据库错误: {}", e),
            UserServiceError::AuthenticationError(msg) => write!(f, "认证错误: {}", msg),
            UserServiceError::RegistrationError(msg) => write!(f, "注册错误: {}", msg),
            UserServiceError::PasswordHashError(e) => write!(f, "密码哈希错误: {}", e),
        }
    }
}

impl From<sqlx::Error> for UserServiceError {
    fn from(error: sqlx::Error) -> Self {
        UserServiceError::DatabaseError(error)
    }
}

impl From<argon2::password_hash::Error> for UserServiceError {
    fn from(error: argon2::password_hash::Error) -> Self {
        UserServiceError::PasswordHashError(error)
    }
}

pub struct UserService {
    user_repo: UserRepository,
}

impl UserService {
    pub fn new(pool: MySqlPool) -> Self {
        Self {
            user_repo: UserRepository::new(pool),
        }
    }

    /// 用户登录
    pub async fn login(&self, payload: &LoginRequest) -> Result<User, UserServiceError> {
        // 查询用户信息
        let user = self.user_repo.find_by_identifier(&payload.identifier).await?;

        let user = match user {
            Some(user) => user,
            None => return Err(UserServiceError::AuthenticationError("用户名或密码错误".to_string())),
        };

        // 验证密码
        if !self.verify_password(&payload.password, &user.password_hash) {
            return Err(UserServiceError::AuthenticationError("用户名或密码错误".to_string()));
        }

        Ok(user)
    }

    /// 用户注册
    pub async fn register(&self, payload: &RegisterRequest) -> Result<User, UserServiceError> {
        // 检查用户是否已存在
        let exists = self.user_repo.check_user_exists(
            &payload.username, 
            &payload.email, 
            &payload.phone
        ).await?;

        if exists {
            return Err(UserServiceError::RegistrationError("用户名、邮箱或手机号已被注册".to_string()));
        }

        // 哈希密码
        let hashed_password = self.hash_password(&payload.password)?;

        // 插入新用户
        self.user_repo.create_user(
            &payload.username,
            &hashed_password,
            &payload.email,
            &payload.phone,
        ).await?;

        // 查询刚创建的用户
        let user = self.user_repo.find_by_username(&payload.username).await?;

        match user {
            Some(user) => Ok(user),
            None => Err(UserServiceError::RegistrationError("注册成功但获取用户信息失败".to_string())),
        }
    }

    fn hash_password(&self, password: &str) -> Result<String, argon2::password_hash::Error> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;
        Ok(password_hash.to_string())
    }

    fn verify_password(&self, password: &str, hashed_password: &str) -> bool {
        if let Ok(parsed_hash) = PasswordHash::new(hashed_password) {
            let argon2 = Argon2::default();
            argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok()
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use argon2::{
        password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
        Argon2,
    };

    fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;
        Ok(password_hash.to_string())
    }

    fn verify_password(password: &str, hashed_password: &str) -> bool {
        if let Ok(parsed_hash) = PasswordHash::new(hashed_password) {
            let argon2 = Argon2::default();
            argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok()
        } else {
            false
        }
    }

    #[test]
    fn test_hash_password() {
        let password = "test_password";
        let hashed = hash_password(password).expect("Failed to hash password");
        
        assert!(verify_password(password, &hashed));
        assert!(!verify_password("wrong_password", &hashed));
    }
}