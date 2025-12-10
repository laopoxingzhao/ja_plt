use std::env;
use std::net::SocketAddr;
use std::result::Result;

use axum::{
    Router,
    extract::Request,
    middleware::{self, Next},
    response::Response,
    routing::get,
};
use jz::{log_init, init_db_pool, user_routes, service_routes, order_routes, init_app_state, AppState};
use std::time::Instant;
use tower_http::services::ServeDir;
// 添加Arc用于共享状态
use std::sync::Arc;
// 添加sqlx导入
use sqlx::mysql::MySqlPool;
// 添加cors支持
use tower_http::cors::{CorsLayer, Any};

/// 应用程序主入口点
/// #[tokio::main] 宏将异步 main 函数包装为同步代码
/// 使用 tokio 多线程运行时
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    log_init();


    // 初始化应用状态
    let app_state = init_app_state();

    // 初始化数据库连接池
    let pool = init_db_pool(&app_state.database_url).await
        .expect("Failed to initialize database pool");

    // let _database_url = env::var("DATABASE_URL")?;
    
    
    // 创建嵌套路由，并将数据库连接池作为状态传递给这些路由
    let api_routes = Router::new()
        .nest("/users", user_routes())
        .nest("/services", service_routes())
        .nest("/orders", order_routes())
        .with_state(pool) // 为API路由提供数据库连接池
        .route_layer(middleware::from_fn(auth_interceptor));

    // 静态文件服务
    let serve_dir = ServeDir::new("assets").append_index_html_on_directories(true);

    let app = Router::new()
        .route("/", get(|| async { "欢迎使用家政服务API" }))
        .nest("/api", api_routes) // 将所有 API 路由嵌套在 /api 路径下
        // 添加CORS支持
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any))
        .fallback_service(serve_dir)
        .with_state(Arc::new(app_state));

    // 从环境变量读取主机和端口，默认为 0.0.0.0:8000
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8000".to_string());
    let addr: SocketAddr = format!("{}:{}", host, port).parse()?;

    println!("🚀 正在家政服务API服务器 {}", addr);
    println!("📖 文档地址: http://{}:{}/docs", host, port);
    println!("📁 静态文件服务: http://{}:{}/assets/", host, port);

    // 绑定到地址并启动服务器
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

// 不再需要独立的 handle_404 函数，因为 ServeDir 已经内置了 not_found_service 处理
// 且我们通过 handle_error 处理了文件读取错误

async fn logging_interceptor(
    request: Request,
    next: Next,
) -> Result<Response, axum::http::StatusCode> {
    let start_time = Instant::now();
    let method = request.method().clone();
    let uri = request.uri().clone();

    // 前置处理 - preHandle
    tracing::info!("开始处理请求:logging {} {}", method, uri);

    // 执行实际请求处理
    let response = next.run(request).await;

    // 后置处理 - postHandle
    let duration = start_time.elapsed();
    tracing::info!(
        "请求完成:logging {} {} - 状态码: {} - 耗时: {:?}",
        method,
        uri,
        response.status(),
        duration
    );

    Ok(response)
}

// 404处理器

// 只对特定路径应用拦截器
async fn auth_interceptor(
    request: Request,
    next: Next,
) -> Result<Response, axum::http::StatusCode> {
    let path = request.uri().path();

    // 对特定路径进行权限检查
    if path.starts_with("/api") || path.starts_with("/orders") {
        // 模拟权限验证
        let auth_header = request.headers().get("authorization");
        if auth_header.is_none() {
            return Err(axum::http::StatusCode::UNAUTHORIZED);
        }
        tracing::info!("权限验证通过: {}", path);
    }

    let response = next.run(request).await;
    Ok(response)
}