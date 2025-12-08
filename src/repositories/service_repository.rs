//! 服务数据访问层
//!
//! 负责服务相关的数据库操作

use sqlx::mysql::MySqlPool;

pub struct ServiceRepository {
    pool: MySqlPool,
}

impl ServiceRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
    
    // TODO: 实现服务相关的数据库操作
}