//! 订单业务逻辑层
//!
//! 处理订单相关的业务逻辑

use sqlx::mysql::MySqlPool;
use crate::repositories::OrderRepository;

pub struct OrderService {
    order_repo: OrderRepository,
}

impl OrderService {
    pub fn new(pool: MySqlPool) -> Self {
        Self {
            order_repo: OrderRepository::new(pool),
        }
    }
    
    // TODO: 实现订单相关的业务逻辑
}