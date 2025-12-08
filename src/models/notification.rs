//! 通知相关模型
//!
//! 对应数据库中的 notifications 表

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 通知模型
/// 对应 notifications 表
/// 
/// 存储发送给用户的消息通知
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    /// 消息ID (主键)
    pub notification_id: i32,
    
    /// 用户ID (外键)
    pub user_id: i32,
    
    /// 消息类型，枚举值:
    /// - "order": 订单相关
    /// - "system": 系统消息
    /// - "promotion": 促销消息
    pub notification_type: String,
    
    /// 消息标题
    pub title: String,
    
    /// 消息内容
    pub content: String,
    
    /// 关联业务ID (可选)
    pub related_id: Option<String>,
    
    /// 是否已读
    pub is_read: bool,
    
    /// 消息创建时间
    pub created_at: chrono::NaiveDateTime,
}