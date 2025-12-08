// 集成测试入口文件

mod user_tests;
mod auth_tests;
mod service_tests;

// 测试辅助函数
#[cfg(test)]
pub mod test_helpers {
    use sqlx::mysql::MySqlPool;
    use std::env;

    pub async fn setup_test_db() -> MySqlPool {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        MySqlPool::connect(&database_url).await.expect("Failed to connect to test database")
    }
}