pub mod models;
pub mod utils;
pub mod config;
pub mod handler;
pub mod database;
pub mod services;
pub mod repositories;

// 重新导出主要模块，方便在main.rs和其他crate中使用
pub use config::{log_init, AppState, init_app_state};
pub use database::init_db_pool;
pub use handler::{user_routes, service_routes, order_routes};