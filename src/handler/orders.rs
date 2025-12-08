use axum::{
    routing::get,
    Router,
    extract::{State, Path},
};
use sqlx::mysql::MySqlPool;
use crate::services::OrderService;

pub fn routes() -> Router<MySqlPool> {
    Router::new()
        .route("/", get(list_orders))
        .route("/{id}", get(get_order))
}

pub async fn list_orders(State(_pool): State<MySqlPool>) -> String {
    // let _order_service = OrderService::new(pool);
    // TODO: 实现真实的业务逻辑
    "列出所有订单".to_string()
}

pub async fn get_order(State(_pool): State<MySqlPool>, Path(id): Path<String>) -> String {
    // let _order_service = OrderService::new(pool);
    // TODO: 实现真实的业务逻辑
    format!("获取单个订单，ID: {}", id)
}