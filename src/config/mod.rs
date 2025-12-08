mod log_config;

use std::sync::OnceLock;

// use log_config::log_init;
use tracing_appender::non_blocking::WorkerGuard;


static LOG_GUARD: OnceLock<WorkerGuard> = OnceLock::new();
pub fn log_init(){
   LOG_GUARD.get_or_init(|| log_config::log_init());
}