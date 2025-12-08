//! 服务业务逻辑层
//!
//! 处理服务相关的业务逻辑

use sqlx::mysql::MySqlPool;
use crate::repositories::ServiceRepository;

pub struct ServiceService {
    service_repo: ServiceRepository,
}

impl ServiceService {
    pub fn new(pool: MySqlPool) -> Self {
        Self {
            service_repo: ServiceRepository::new(pool),
        }
    }
    
    // TODO: 实现服务相关的业务逻辑
}