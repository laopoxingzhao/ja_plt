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