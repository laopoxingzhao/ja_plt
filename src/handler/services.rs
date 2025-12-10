use axum::{
    routing::{get, post},
    Router,
    extract::{State, Path},
    Json, http::StatusCode,
};
use sqlx::mysql::MySqlPool;
use crate::{services::service_service::ServiceService, models::service::Service};

pub fn routes() -> Router<MySqlPool> {
    Router::new()
        .route("/", get(list_services))
        .route("/{id}", get(get_service))
}

pub async fn list_services(State(pool): State<MySqlPool>) -> Result<Json<Vec<Service>>, (StatusCode, String)> {
    let service_service = ServiceService::new(pool);
    match service_service.list_services().await {
        Ok(services) => Ok(Json(services)),
        Err(e) => {
            tracing::error!("获取服务列表错误: {:?}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, "服务器内部错误".to_string()))
        }
    }
}

pub async fn get_service(
    State(pool): State<MySqlPool>,
    Path(id): Path<i32>
) -> Result<Json<Service>, (StatusCode, String)> {
    let service_service = ServiceService::new(pool);
    match service_service.get_service_by_id(id).await {
        Ok(service) => Ok(Json(service)),
        Err(sqlx::Error::RowNotFound) => {
            Err((StatusCode::NOT_FOUND, "服务不存在".to_string()))
        }
        Err(e) => {
            tracing::error!("获取服务错误: {:?}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, "服务器内部错误".to_string()))
        }
    }
}