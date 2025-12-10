use axum::{
    routing::{get, post},
    Router,
    extract::{State, Path},
    Json, http::StatusCode,
};
use sqlx::mysql::MySqlPool;
use crate::{services::order_service::OrderService, models::order::Order};

pub fn routes() -> Router<MySqlPool> {
    Router::new()
        .route("/", get(list_orders))
        .route("/{id}", get(get_order))
}

pub async fn list_orders(State(pool): State<MySqlPool>) -> Result<Json<Vec<Order>>, (StatusCode, String)> {
    let order_service = OrderService::new(pool);
    match order_service.list_orders().await {
        Ok(orders) => Ok(Json(orders)),
        Err(e) => {
            tracing::error!("获取订单列表错误: {:?}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, "服务器内部错误".to_string()))
        }
    }
}

pub async fn get_order(
    State(pool): State<MySqlPool>,
    Path(id): Path<String>
) -> Result<Json<Order>, (StatusCode, String)> {
    let order_service = OrderService::new(pool);
    match order_service.get_order_by_id(id).await {
        Ok(order) => Ok(Json(order)),
        Err(sqlx::Error::RowNotFound) => {
            Err((StatusCode::NOT_FOUND, "订单不存在".to_string()))
        }
        Err(e) => {
            tracing::error!("获取订单错误: {:?}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, "服务器内部错误".to_string()))
        }
    }
}