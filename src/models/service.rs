//! 服务相关模型
//!
//! 对应数据库中的 service_categories, services 和 service_addons 表

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 服务分类模型
/// 对应 service_categories 表
/// 
/// 定义家政服务的分类体系，支持层级分类结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceCategory {
    /// 分类唯一标识符 (主键)
    pub category_id: i32,
    
    /// 分类名称
    pub category_name: String,
    
    /// 父分类ID (可选，用于构建分类树)
    pub parent_id: Option<i32>,
    
    /// 分类图标URL (可选)
    pub icon_url: Option<String>,
    
    /// 分类是否启用
    pub is_active: bool,
    
    /// 分类显示排序
    pub sort_order: i32,
    
    /// 分类创建时间
    pub created_at: chrono::NaiveDateTime,
}

/// 服务项目模型
/// 对应 services 表
/// 
/// 定义具体的服务项目信息，关联到某个服务分类
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Service {
    /// 服务唯一标识符 (主键)
    pub service_id: i32,
    
    /// 所属分类ID (外键)
    pub category_id: i32,
    
    /// 服务名称
    pub service_name: String,
    
    /// 服务详细描述 (可选)
    pub description: Option<String>,
    
    /// 基础价格
    pub base_price: f64,
    
    /// 计价单位，枚举值:
    /// - "hour": 按小时计费
    /// - "square_meter": 按平方米计费
    /// - "item": 按项目计费
    /// - "fixed": 固定价格
    pub unit: String,
    
    /// 最小时长/数量
    pub min_duration: i32,
    
    /// 最大时长/数量
    pub max_duration: i32,
    
    /// 服务是否启用
    pub is_active: bool,
    
    /// 服务创建时间
    pub created_at: chrono::NaiveDateTime,
}

/// 服务附加项模型
/// 对应 service_addons 表
/// 
/// 定义服务的可选项或增值服务，可附加到主服务上
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceAddon {
    /// 附加项唯一标识符 (主键)
    pub addon_id: i32,
    
    /// 关联的服务ID (外键)
    pub service_id: i32,
    
    /// 附加项名称
    pub addon_name: String,
    
    /// 附加项价格
    pub addon_price: f64,
    
    /// 附加项是否启用
    pub is_active: bool,
    
    /// 附加项创建时间
    pub created_at: chrono::NaiveDateTime,
}