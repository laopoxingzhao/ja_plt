//! 订单相关模型
//!
//! 对应数据库中的 orders 和 order_addons 表

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 订单模型
/// 对应 orders 表
/// 
/// 存储用户订单的完整信息，包括服务详情、价格、状态等
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    /// 订单号，唯一标识符
    pub order_id: String,
    
    /// 客户ID (外键)
    pub customer_id: i32,
    
    /// 服务人员ID (外键，可选)
    pub worker_id: Option<i32>,
    
    /// 地址ID (外键)
    pub address_id: i32,
    
    /// 服务ID (外键)
    pub service_id: i32,
    
    /// 优惠券ID (外键，可选)
    pub coupon_id: Option<i32>,
    
    /// 服务日期
    pub service_date: chrono::NaiveDate,
    
    /// 服务时段，枚举值:
    /// - "morning": 上午
    /// - "afternoon": 下午
    /// - "evening": 晚上
    /// - "full_day": 全天
    pub time_slot: String,
    
    /// 服务时长
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
    
    /// 计划开始时间 (可选)
    pub scheduled_start_time: Option<chrono::NaiveDateTime>,
    
    /// 实际开始时间 (可选)
    pub actual_start_time: Option<chrono::NaiveDateTime>,
    
    /// 实际结束时间 (可选)
    pub actual_end_time: Option<chrono::NaiveDateTime>,
    
    /// 订单创建时间
    pub created_at: chrono::NaiveDateTime,
    
    /// 订单更新时间
    pub updated_at: chrono::NaiveDateTime,
}

/// 订单附加项模型
/// 对应 order_addons 表
/// 
/// 记录订单中选择的附加服务项目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderAddon {
    /// 订单附加项ID (主键)
    pub order_addon_id: i32,
    
    /// 关联订单ID (外键)
    pub order_id: String,
    
    /// 附加项ID (外键)
    pub addon_id: i32,
    
    /// 数量
    pub quantity: i32,
    
    /// 单价
    pub unit_price: f64,
    
    /// 创建时间
    pub created_at: chrono::NaiveDateTime,
}