use axum::{
    routing::{get, post},
    Router, Json, http::StatusCode,
    extract::State,
};
use std::env;

use crate::models::auth::{LoginRequest, LoginResponse, RegisterRequest, RegisterResponse};
use crate::utils::jwt::Claims;
use crate::services::UserService;
use crate::services::user_service::UserServiceError;

pub fn routes() -> Router<sqlx::mysql::MySqlPool> {
    Router::new()
        .route("/", get(list_users))
        .route("/user/{id}", get(get_user))  // 修复：使用Axum v0.8的路径参数格式
        .route("/login", post(login))
        .route("/register", post(register))
}

pub async fn list_users(State(_pool): State<sqlx::mysql::MySqlPool>) -> &'static str {
    // TODO: 实现真实的数据库查询逻辑
    "列出所有用户"
}

pub async fn get_user(State(_pool): State<sqlx::mysql::MySqlPool>, id: String) -> String {
    // TODO: 实现真实的数据库查询逻辑
    format!("获取单个用户，ID: {}", id)
    let user_service = UserService::new(pool);
    user_service.get_user(id).await 

}

/// 登录接口
/// 接受用户名/邮箱和密码，验证成功后返回JWT token
pub async fn login(
    State(pool): State<sqlx::mysql::MySqlPool>,
    Json(payload): Json<LoginRequest>
) -> Result<Json<LoginResponse>, (StatusCode, String)> {
    let user_service = UserService::new(pool);
    
    match user_service.login(&payload).await {
        Ok(user) => {
            // 获取JWT密钥
            let jwt_secret = env::var("JWT_SECRET")
                .unwrap_or_else(|_| "secret-development-key".to_string());
            
            // 生成JWT token
            let claims = Claims::new(&user);
            let token = claims.generate_token(&jwt_secret)
                .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "无法生成访问令牌".to_string()))?;
            
            let response = LoginResponse {
                token,
                user,
            };
            
            Ok(Json(response))
        }
        Err(UserServiceError::AuthenticationError(msg)) => {
            Err((StatusCode::UNAUTHORIZED, msg))
        }
        Err(e) => {
            tracing::error!("登录错误: {:?}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, "服务器内部错误".to_string()))
        }
    }
}

/// 注册接口
/// 接受用户名、邮箱、手机号和密码，创建新用户
pub async fn register(
    State(pool): State<sqlx::mysql::MySqlPool>,
    Json(payload): Json<RegisterRequest>
) -> Result<(StatusCode, Json<RegisterResponse>), (StatusCode, String)> {
    let user_service = UserService::new(pool);
    
    match user_service.register(&payload).await {
        Ok(user) => {
            let response = RegisterResponse {
                user,
            };
            
            Ok((StatusCode::CREATED, Json(response)))
        }
        Err(UserServiceError::RegistrationError(msg)) => {
            Err((StatusCode::BAD_REQUEST, msg))
        }
        Err(e) => {
            tracing::error!("注册错误: {:?}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, "服务器内部错误".to_string()))
        }
    }
}