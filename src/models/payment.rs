//! 支付相关模型
//!
//! 对应数据库中的 payments 表

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 支付模型
/// 对应 payments 表
/// 
/// 记录订单的支付信息和状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payment {
    /// 支付ID，唯一标识符
    pub payment_id: String,
    
    /// 关联订单ID (外键)
    pub order_id: String,
    
    /// 用户ID (外键)
    pub user_id: i32,
    
    /// 支付方式，枚举值:
    /// - "wechat": 微信支付
    /// - "alipay": 支付宝
    /// - "balance": 余额支付
    /// - "card": 银行卡支付
    pub payment_method: String,
    
    /// 支付金额
    pub payment_amount: f64,
    
    /// 支付状态，枚举值:
    /// - "pending": 待支付
    /// - "processing": 处理中
    /// - "success": 支付成功
    /// - "failed": 支付失败
    pub payment_status: String,
    
    /// 第三方交易号 (可选)
    pub thirdparty_trade_no: Option<String>,
    
    /// 支付时间 (可选)
    pub payment_time: Option<chrono::NaiveDateTime>,
    
    /// 支付记录创建时间
    pub created_at: chrono::NaiveDateTime,
}