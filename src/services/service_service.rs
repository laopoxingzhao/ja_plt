//! 服务业务逻辑层
//!
//! 处理服务相关的业务逻辑

use sqlx::mysql::MySqlPool;
use crate::repositories::ServiceRepository;
use crate::models::service::Service;

pub struct ServiceService {
    service_repo: ServiceRepository,
}

impl ServiceService {
    pub fn new(pool: MySqlPool) -> Self {
        Self {
            service_repo: ServiceRepository::new(pool),
        }
    }
    
    /// 获取服务列表
    pub async fn list_services(&self) -> Result<Vec<Service>, sqlx::Error> {
        self.service_repo.list_services().await
    }
    
    /// 根据ID获取服务
    pub async fn get_service_by_id(&self, id: i32) -> Result<Service, sqlx::Error> {
        self.service_repo.find_by_id(id).await
    }
}