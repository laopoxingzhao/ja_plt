//! 订单业务逻辑层
//!
//! 处理订单相关的业务逻辑

use sqlx::mysql::MySqlPool;
use crate::repositories::OrderRepository;
use crate::models::order::Order;

pub struct OrderService {
    order_repo: OrderRepository,
}

impl OrderService {
    pub fn new(pool: MySqlPool) -> Self {
        Self {
            order_repo: OrderRepository::new(pool),
        }
    }
    
    /// 获取订单列表
    pub async fn list_orders(&self) -> Result<Vec<Order>, sqlx::Error> {
        self.order_repo.list_orders().await
    }
    
    /// 根据ID获取订单
    pub async fn get_order_by_id(&self, id: String) -> Result<Order, sqlx::Error> {
        self.order_repo.find_by_id(id).await
    }
}