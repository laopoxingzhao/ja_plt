//! 订单数据访问层
//!
//! 负责订单相关的数据库操作

use sqlx::{mysql::MySqlPool, Row};
use crate::models::order::Order;

pub struct OrderRepository {
    pool: MySqlPool,
}

impl OrderRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
    
    /// 获取所有订单列表
    pub async fn list_orders(&self) -> Result<Vec<Order>, sqlx::Error> {
        let orders_result = sqlx::query(
            "SELECT order_id, customer_id, worker_id, address_id, service_id, coupon_id, \
            service_date, time_slot, duration, unit_price, subtotal, discount_amount, \
            total_amount, payment_status, order_status, special_instructions, \
            cancellation_reason, scheduled_start_time, actual_start_time, actual_end_time, \
            created_at, updated_at \
            FROM orders"
        )
        .fetch_all(&self.pool)
        .await;

        match orders_result {
            Ok(rows) => {
                let orders: Vec<Order> = rows.into_iter().map(|row| {
                    Order {
                        order_id: row.get("order_id"),
                        customer_id: row.get("customer_id"),
                        worker_id: row.get("worker_id"),
                        address_id: row.get("address_id"),
                        service_id: row.get("service_id"),
                        coupon_id: row.get("coupon_id"),
                        service_date: row.get("service_date"),
                        time_slot: row.get("time_slot"),
                        duration: row.get("duration"),
                        unit_price: row.get("unit_price"),
                        subtotal: row.get("subtotal"),
                        discount_amount: row.get("discount_amount"),
                        total_amount: row.get("total_amount"),
                        payment_status: row.get("payment_status"),
                        order_status: row.get("order_status"),
                        special_instructions: row.get("special_instructions"),
                        cancellation_reason: row.get("cancellation_reason"),
                        scheduled_start_time: row.get("scheduled_start_time"),
                        actual_start_time: row.get("actual_start_time"),
                        actual_end_time: row.get("actual_end_time"),
                        created_at: row.get("created_at"),
                        updated_at: row.get("updated_at"),
                    }
                }).collect();
                Ok(orders)
            }
            Err(e) => Err(e),
        }
    }
    
    /// 根据ID查找订单
    pub async fn find_by_id(&self, id: String) -> Result<Order, sqlx::Error> {
        let order_result = sqlx::query(
            "SELECT order_id, customer_id, worker_id, address_id, service_id, coupon_id, \
            service_date, time_slot, duration, unit_price, subtotal, discount_amount, \
            total_amount, payment_status, order_status, special_instructions, \
            cancellation_reason, scheduled_start_time, actual_start_time, actual_end_time, \
            created_at, updated_at \
            FROM orders WHERE order_id = ?"
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await;

        match order_result {
            Ok(row) => {
                let order = Order {
                    order_id: row.get("order_id"),
                    customer_id: row.get("customer_id"),
                    worker_id: row.get("worker_id"),
                    address_id: row.get("address_id"),
                    service_id: row.get("service_id"),
                    coupon_id: row.get("coupon_id"),
                    service_date: row.get("service_date"),
                    time_slot: row.get("time_slot"),
                    duration: row.get("duration"),
                    unit_price: row.get("unit_price"),
                    subtotal: row.get("subtotal"),
                    discount_amount: row.get("discount_amount"),
                    total_amount: row.get("total_amount"),
                    payment_status: row.get("payment_status"),
                    order_status: row.get("order_status"),
                    special_instructions: row.get("special_instructions"),
                    cancellation_reason: row.get("cancellation_reason"),
                    scheduled_start_time: row.get("scheduled_start_time"),
                    actual_start_time: row.get("actual_start_time"),
                    actual_end_time: row.get("actual_end_time"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                };
                Ok(order)
            }
            Err(e) => Err(e),
        }
    }
}