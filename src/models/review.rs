//! 评价与投诉相关模型
//!
//! 对应数据库中的 reviews 和 complaints 表

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 评价模型
/// 对应 reviews 表
/// 
/// 存储用户对服务的评价信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Review {
    /// 评价ID (主键)
    pub review_id: i32,
    
    /// 关联订单ID (外键)
    pub order_id: String,
    
    /// 客户ID (外键)
    pub customer_id: i32,
    
    /// 服务人员ID (外键)
    pub worker_id: i32,
    
    /// 总评分 (1-5分)
    pub rating: i8,
    
    /// 服务评分 (可选)
    pub service_rating: Option<i8>,
    
    /// 守时评分 (可选)
    pub punctuality_rating: Option<i8>,
    
    /// 评价内容 (可选)
    pub review_text: Option<String>,
    
    /// 是否匿名评价
    pub is_anonymous: bool,
    
    /// 评价创建时间
    pub created_at: chrono::NaiveDateTime,
}

/// 投诉模型
/// 对应 complaints 表
/// 
/// 存储用户提交的投诉信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Complaint {
    /// 投诉ID (主键)
    pub complaint_id: i32,
    
    /// 关联订单ID (外键)
    pub order_id: String,
    
    /// 投诉人ID (外键)
    pub complainant_id: i32,
    
    /// 被投诉人ID (外键)
    pub target_id: i32,
    
    /// 投诉类型，枚举值:
    /// - "service": 服务质量
    /// - "punctuality": 守时问题
    /// - "attitude": 服务态度
    /// - "other": 其他问题
    pub complaint_type: String,
    
    /// 投诉内容
    pub complaint_text: String,
    
    /// 处理状态，枚举值:
    /// - "pending": 待处理
    /// - "investigating": 调查中
    /// - "resolved": 已解决
    /// - "rejected": 已驳回
    pub status: String,
    
    /// 处理时间 (可选)
    pub resolved_at: Option<chrono::NaiveDateTime>,
    
    /// 投诉创建时间
    pub created_at: chrono::NaiveDateTime,
}