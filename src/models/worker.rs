//! 服务人员相关模型
//!
//! 对应数据库中的 worker_profiles 和 worker_schedules 表

use serde::{Deserialize, Serialize};

/// 服务人员详情模型
/// 对应 worker_profiles 表
/// 
/// 存储服务人员的详细资料和专业信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerProfile {
    /// 服务人员ID，与用户ID一致 (主键)
    pub worker_id: i32,
    
    /// 服务分类ID (外键)
    pub service_category_id: i32,
    
    /// 时薪
    pub hourly_rate: f64,
    
    /// 个人简介 (可选)
    pub bio: Option<String>,
    
    /// 技能标签列表 (JSON格式)
    pub skills: Option<Vec<String>>,
    
    /// 服务区域列表 (JSON格式)
    pub service_area: Option<Vec<String>>,
    
    /// 总订单数
    pub total_orders: i32,
    
    /// 已完成订单数
    pub completed_orders: i32,
    
    /// 平均评分
    pub avg_rating: f64,
    
    /// 是否可接单
    pub is_available: bool,
    
    /// 每日最大接单数
    pub max_daily_orders: i32,
    
    /// 资料创建时间
    pub created_at: chrono::NaiveDateTime,
    
    /// 资料更新时间
    pub updated_at: chrono::NaiveDateTime,
}

/// 服务人员日程模型
/// 对应 worker_schedules 表
/// 
/// 管理服务人员的工作时间安排
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerSchedule {
    /// 日程唯一标识符 (主键)
    pub schedule_id: i32,
    
    /// 服务人员ID (外键)
    pub worker_id: i32,
    
    /// 排班日期
    pub schedule_date: chrono::NaiveDate,
    
    /// 时间段，枚举值:
    /// - "morning": 上午
    /// - "afternoon": 下午
    /// - "evening": 晚上
    /// - "full_day": 全天
    pub time_slot: String,
    
    /// 状态，枚举值:
    /// - "available": 可用
    /// - "booked": 已预订
    /// - "unavailable": 不可用
    pub status: String,
    
    /// 关联订单ID (可选)
    pub order_id: Option<String>,
    
    /// 日程创建时间
    pub created_at: chrono::NaiveDateTime,
}