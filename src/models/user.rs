//! 用户相关模型
//!
//! 对应数据库中的 users 和 user_addresses 表

use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;

/// 用户模型
/// 对应 users 表
/// 
/// users 表存储系统中的所有用户信息，包括客户、服务人员和管理员
/// 用户可以通过不同角色参与系统活动
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// 用户唯一标识符 (主键)
    pub user_id: i32,
    
    /// 用户名，系统内唯一
    pub username: String,
    
    /// 经过 Argon2 哈希处理的密码
    pub password_hash: String,
    
    /// 用户邮箱，系统内唯一
    pub email: String,
    
    /// 用户手机号，系统内唯一
    pub phone: String,
    
    /// 用户类型，枚举值:
    /// - "customer": 普通客户
    /// - "worker": 服务人员
    /// - "admin": 系统管理员
    #[serde(rename = "type")]
    pub user_type: String,
    
    /// 用户头像图片URL (可选)
    pub avatar_url: Option<String>,
    
    /// 用户真实姓名 (可选)
    pub real_name: Option<String>,
    
    /// 是否已完成实名认证
    pub is_verified: bool,
    
    /// 用户账户余额
    pub balance: f64,
    
    /// 账户状态，枚举值:
    /// - "active": 活跃账户
    /// - "inactive": 非活跃账户
    /// - "banned": 已封禁账户
    pub status: String,
    
    /// 账户创建时间
    pub created_at: NaiveDateTime,
    
    /// 账户信息最后更新时间
    pub updated_at: NaiveDateTime,
}

/// 用户地址模型
/// 对应 user_addresses 表
/// 
/// 存储用户的常用服务地址信息，用户可以设置默认地址
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAddress {
    /// 地址唯一标识符 (主键)
    pub address_id: i32,
    
    /// 关联的用户ID (外键)
    pub user_id: i32,
    
    /// 联系人姓名
    pub contact_name: String,
    
    /// 联系电话
    pub contact_phone: String,
    
    /// 省份
    pub province: String,
    
    /// 城市
    pub city: String,
    
    /// 区/县
    pub district: String,
    
    /// 详细街道地址
    pub street_address: String,
    
    /// 是否为默认地址
    pub is_default: bool,
    
    /// 地址创建时间
    pub created_at: NaiveDateTime,
}