//! 订单数据访问层
//!
//! 负责订单相关的数据库操作

use sqlx::mysql::MySqlPool;

pub struct OrderRepository {
    pool: MySqlPool,
}

impl OrderRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
    
    // TODO: 实现订单相关的数据库操作
}