mod log_config;
mod env_config;

use std::sync::OnceLock;

// use log_config::log_init;
use tracing_appender::non_blocking::WorkerGuard;

static LOG_GUARD: OnceLock<WorkerGuard> = OnceLock::new();

pub struct AppState {
    pub database_url: String,
    pub jwt_secret: String,
}

pub fn init_app_state() -> AppState {
    AppState {
        database_url: env_config::database_url(),
        jwt_secret: env_config::jwt_secret(),
    }
}

pub fn log_init() {
    LOG_GUARD.get_or_init(|| log_config::log_init());
}