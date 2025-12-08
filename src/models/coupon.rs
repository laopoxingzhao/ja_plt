//! 优惠券相关模型
//!
//! 对应数据库中的 coupons 和 user_coupons 表

use serde::{Deserialize, Serialize};

/// 优惠券模型
/// 对应 coupons 表
/// 
/// 定义系统中可使用的优惠券模板
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Coupon {
    /// 优惠券ID (主键)
    pub coupon_id: i32,
    
    /// 优惠券码，唯一标识符
    pub coupon_code: String,
    
    /// 优惠券名称
    pub coupon_name: String,
    
    /// 折扣类型，枚举值:
    /// - "percentage": 百分比折扣
    /// - "fixed": 固定金额折扣
    /// - "service": 服务折扣
    pub discount_type: String,
    
    /// 折扣值
    pub discount_value: f64,
    
    /// 最低订单金额要求
    pub min_order_amount: f64,
    
    /// 适用服务列表 (JSON格式)
    pub applicable_services: Option<Vec<i32>>,
    
    /// 有效期开始日期
    pub valid_from: chrono::NaiveDate,
    
    /// 有效期截止日期
    pub valid_until: chrono::NaiveDate,
    
    /// 使用次数限制
    pub usage_limit: i32,
    
    /// 已使用次数
    pub used_count: i32,
    
    /// 优惠券是否启用
    pub is_active: bool,
    
    /// 创建时间
    pub created_at: chrono::NaiveDateTime,
}

/// 用户优惠券模型
/// 对应 user_coupons 表
/// 
/// 记录用户获得的优惠券及其使用情况
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserCoupon {
    /// 用户优惠券ID (主键)
    pub user_coupon_id: i32,
    
    /// 用户ID (外键)
    pub user_id: i32,
    
    /// 优惠券ID (外键)
    pub coupon_id: i32,
    
    /// 关联订单ID (外键，可选)
    pub order_id: Option<String>,
    
    /// 是否已使用
    pub is_used: bool,
    
    /// 使用时间 (可选)
    pub used_at: Option<chrono::NaiveDateTime>,
    
    /// 过期时间
    pub expires_at: chrono::NaiveDate,
    
    /// 创建时间
    pub created_at: chrono::NaiveDateTime,
}