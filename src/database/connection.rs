//! 数据库连接模块
//!
//! 负责初始化和管理数据库连接池

use sqlx::mysql::MySqlPool;
use std::env;

/// 初始化数据库连接池
pub async fn init_db_pool(database_url : &str) -> Result<MySqlPool, sqlx::Error> {
    
    let pool = MySqlPool::connect(&database_url).await?;
    
    Ok(pool)    
}