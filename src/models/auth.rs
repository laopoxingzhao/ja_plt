//! 认证相关模型
//!
//! 包含登录和注册等认证流程所需的数据传输对象

use serde::{Deserialize, Serialize};
use crate::models::user::User;

/// 登录请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    /// 用户名或邮箱
    pub identifier: String,
    
    /// 密码
    pub password: String,
}

/// 登录响应结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResponse {
    /// JWT Token
    pub token: String,
    
    /// 刷新令牌
    pub refresh_token: String,
    
    /// 用户信息
    pub user: User,
}

/// 注册请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterRequest {
    /// 用户名
    pub username: String,
    
    /// 邮箱
    pub email: String,
    
    /// 手机号
    pub phone: String,
    
    /// 密码
    pub password: String,
}

/// 注册响应结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterResponse {
    /// 用户信息
    pub user: User,
}

/// 刷新令牌请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshTokenRequest {
    /// 刷新令牌
    pub refresh_token: String,
}

/// 刷新令牌响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshTokenResponse {
    /// 新的JWT Token
    pub token: String,
    
    /// 新的刷新令牌
    pub refresh_token: String,
}

/// 登出请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogoutRequest {
    /// 刷新令牌
    pub refresh_token: String,
}