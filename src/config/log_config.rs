use std::fs;

use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::fmt;
use tracing_subscriber::prelude::*;
use tracing_subscriber::registry::Registry;
pub fn log_init() -> WorkerGuard{
    let logs_dir = "logs";
    if let Err(e) = fs::create_dir_all(logs_dir) {
        eprintln!("无法创建日志目录 {}: {}", logs_dir, e);
    }

    let file_appender = tracing_appender::rolling::daily(logs_dir, "jz.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    let file_layer = fmt::layer()
        .with_writer(non_blocking)
        .with_ansi(false)
        .with_target(false)
        .with_filter(tracing_subscriber::filter::LevelFilter::INFO);

    let stdout_layer = fmt::layer()
        .with_writer(std::io::stdout)
        .with_ansi(true)
        .with_target(false)
        .with_filter(tracing_subscriber::filter::LevelFilter::INFO);

    Registry::default()
        .with(file_layer)
        .with(stdout_layer)
        .init();

    tracing::info!(
        "日志系统已初始化（文件: {}/jz.log，控制台输出已启用）",
        logs_dir
    );
    _guard

}