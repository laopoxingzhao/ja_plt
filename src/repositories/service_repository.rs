//! 服务数据访问层
//!
//! 负责服务相关的数据库操作

use sqlx::{mysql::MySqlPool, Row};
use crate::models::service::Service;

pub struct ServiceRepository {
    pool: MySqlPool,
}

impl ServiceRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
    
    /// 获取所有服务列表
    pub async fn list_services(&self) -> Result<Vec<Service>, sqlx::Error> {
        let services_result = sqlx::query(
            "SELECT service_id, category_id, service_name, description, base_price, unit, \
            min_duration, max_duration, is_active, created_at \
            FROM services"
        )
        .fetch_all(&self.pool)
        .await;

        match services_result {
            Ok(rows) => {
                let services: Vec<Service> = rows.into_iter().map(|row| {
                    Service {
                        service_id: row.get("service_id"),
                        category_id: row.get("category_id"),
                        service_name: row.get("service_name"),
                        description: row.get("description"),
                        base_price: row.get("base_price"),
                        unit: row.get("unit"),
                        min_duration: row.get("min_duration"),
                        max_duration: row.get("max_duration"),
                        is_active: row.get("is_active"),
                        created_at: row.get("created_at"),
                    }
                }).collect();
                Ok(services)
            }
            Err(e) => Err(e),
        }
    }
    
    /// 根据ID查找服务
    pub async fn find_by_id(&self, id: i32) -> Result<Service, sqlx::Error> {
        let service_result = sqlx::query(
            "SELECT service_id, category_id, service_name, description, base_price, unit, \
            min_duration, max_duration, is_active, created_at \
            FROM services WHERE service_id = ?"
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await;

        match service_result {
            Ok(row) => {
                let service = Service {
                    service_id: row.get("service_id"),
                    category_id: row.get("category_id"),
                    service_name: row.get("service_name"),
                    description: row.get("description"),
                    base_price: row.get("base_price"),
                    unit: row.get("unit"),
                    min_duration: row.get("min_duration"),
                    max_duration: row.get("max_duration"),
                    is_active: row.get("is_active"),
                    created_at: row.get("created_at"),
                };
                Ok(service)
            }
            Err(e) => Err(e),
        }
    }
}