// 控制器层 (Controllers)
// 负责处理HTTP请求和响应，调用相应的服务层处理业务逻辑
pub mod users;
pub mod services;
pub mod orders;

use axum::Router;
use sqlx::mysql::MySqlPool;

pub fn user_routes() -> Router<MySqlPool> {
    users::routes()
}

pub fn service_routes() -> Router<MySqlPool> {
    services::routes()
}

pub fn order_routes() -> Router<MySqlPool> {
    orders::routes()
}