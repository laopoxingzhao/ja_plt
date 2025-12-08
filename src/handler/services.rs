use axum::{
    routing::get,
    Router,
    extract::{State, Path},
};
use sqlx::mysql::MySqlPool;
use crate::services::ServiceService;

pub fn routes() -> Router<MySqlPool> {
    Router::new()
        .route("/", get(list_services))
        .route("/{id}", get(get_service))
}

pub async fn list_services(State(_pool): State<MySqlPool>) -> String {
    // let _service_service = ServiceService::new(pool);
    // TODO: 实现真实的业务逻辑
    "列出所有服务".to_string()
}

pub async fn get_service(State(_pool): State<MySqlPool>, Path(id): Path<String>) -> String {
    // let _service_service = ServiceService::new(pool);
    // TODO: 实现真实的业务逻辑
    format!("获取单个服务，ID: {}", id)
}