//! 订单相关模型
//!
//! 对应数据库中的 orders 和 order_addons 表

use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{NaiveDateTime, NaiveDate};

/// 订单模型
/// 对应 orders 表
/// 
/// orders 表存储系统中的所有订单信息
/// 订单状态流转: pending -> confirmed -> assigned -> ongoing -> completed/cancelled
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    /// 订单编号 (主键)
    /// 格式: YYYYMMDDXXXXXX (日期+6位自增序列)
    pub order_id: String,
    
    /// 客户ID (外键关联 users 表)
    pub customer_id: i32,
    
    /// 服务人员ID (外键关联 users 表，可为空)
    pub worker_id: Option<i32>,
    
    /// 地址ID (外键关联 user_addresses 表)
    pub address_id: i32,
    
    /// 服务项目ID (外键关联 services 表)
    pub service_id: i32,
    
    /// 优惠券ID (外键关联 coupons 表，可为空)
    pub coupon_id: Option<i32>,
    
    /// 服务日期
    pub service_date: NaiveDate,
    
    /// 时间段，枚举值:
    /// - "morning": 上午 (09:00-12:00)
    /// - "afternoon": 下午 (12:00-18:00)
    /// - "evening": 晚上 (18:00-22:00)
    /// - "full_day": 全天 (09:00-22:00)
    pub time_slot: String,
    
    /// 服务时长/数量
    pub duration: f64,
    
    /// 单价
    pub unit_price: f64,
    
    /// 小计金额
    pub subtotal: f64,
    
    /// 优惠金额
    pub discount_amount: f64,
    
    /// 总金额
    pub total_amount: f64,
    
    /// 支付状态，枚举值:
    /// - "pending": 待支付
    /// - "paid": 已支付
    /// - "refunded": 已退款
    pub payment_status: String,
    
    /// 订单状态，枚举值:
    /// - "pending": 待确认
    /// - "confirmed": 已确认
    /// - "assigned": 已指派
    /// - "ongoing": 进行中
    /// - "completed": 已完成
    /// - "cancelled": 已取消
    pub order_status: String,
    
    /// 特殊要求 (可选)
    pub special_instructions: Option<String>,
    
    /// 取消原因 (可选)
    pub cancellation_reason: Option<String>,
    
    /// 计划开始时间
    pub scheduled_start_time: Option<NaiveDateTime>,
    
    /// 实际开始时间
    pub actual_start_time: Option<NaiveDateTime>,
    
    /// 实际结束时间
    pub actual_end_time: Option<NaiveDateTime>,
    
    /// 订单创建时间
    pub created_at: NaiveDateTime,
    
    /// 订单更新时间
    pub updated_at: NaiveDateTime,
}

/// 订单附加服务模型
/// 对应 order_addons 表
/// 
/// order_addons 表存储订单选择的附加服务信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderAddon {
    /// 订单附加服务ID (主键)
    pub order_addon_id: i32,
    
    /// 订单ID (外键关联 orders 表)
    pub order_id: String,
    
    /// 附加服务ID (外键关联 service_addons 表)
    pub addon_id: i32,
    
    /// 数量
    pub quantity: i32,
    
    /// 单价
    pub unit_price: f64,
    
    /// 创建时间
    pub created_at: NaiveDateTime,
}