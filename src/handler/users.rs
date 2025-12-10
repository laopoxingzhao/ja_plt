use axum::{
    routing::{get, post},
    Router, Json, http::StatusCode,
    extract::{State, Path},
};
use std::env;

use crate::models::auth::{LoginRequest, LoginResponse, RegisterRequest, RegisterResponse, RefreshTokenRequest, RefreshTokenResponse, LogoutRequest};
use crate::utils::jwt::Claims;
use crate::services::user_service::UserService;
use crate::services::user_service::UserServiceError;

pub fn routes() -> Router<sqlx::mysql::MySqlPool> {
    Router::new()
        .route("/", get(list_users))
        .route("/{id}", get(get_user))  // 修正：使用正确的花括号路径参数格式
        .route("/login", post(login))
        .route("/register", post(register))
        .route("/refresh", post(refresh_token))
        .route("/logout", post(logout))
}

pub async fn list_users(State(pool): State<sqlx::mysql::MySqlPool>) -> Result<Json<Vec<crate::models::user::User>>, (StatusCode, String)> {
    let user_service = UserService::new(pool);
    match user_service.list_users().await {
        Ok(users) => Ok(Json(users)),
        Err(e) => {
            tracing::error!("获取用户列表错误: {:?}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, "服务器内部错误".to_string()))
        }
    }
}

pub async fn get_user(
    State(pool): State<sqlx::mysql::MySqlPool>,
    Path(id): Path<i32>
) -> Result<Json<crate::models::user::User>, (StatusCode, String)> {
    let user_service = UserService::new(pool);
    match user_service.get_user_by_id(id).await {
        Ok(user) => Ok(Json(user)),
        Err(UserServiceError::DatabaseError(sqlx::Error::RowNotFound)) => {
            Err((StatusCode::NOT_FOUND, "用户不存在".to_string()))
        }
        Err(e) => {
            tracing::error!("获取用户错误: {:?}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, "服务器内部错误".to_string()))
        }
    }
}

/// 登录接口
/// 接受用户名/邮箱和密码，验证成功后返回JWT token和刷新令牌
pub async fn login(
    State(pool): State<sqlx::mysql::MySqlPool>,
    Json(payload): Json<LoginRequest>
) -> Result<Json<LoginResponse>, (StatusCode, String)> {
    let user_service = UserService::new(pool);
    
    match user_service.login(&payload).await {
        Ok((token, refresh_token, user)) => {
            let response = LoginResponse {
                token,
                refresh_token,
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

/// 刷新令牌接口
/// 使用刷新令牌获取新的访问令牌
pub async fn refresh_token(
    State(pool): State<sqlx::mysql::MySqlPool>,
    Json(payload): Json<RefreshTokenRequest>
) -> Result<Json<RefreshTokenResponse>, (StatusCode, String)> {
    let user_service = UserService::new(pool);
    
    match user_service.refresh_token(&payload).await {
        Ok((token, refresh_token)) => {
            let response = RefreshTokenResponse {
                token,
                refresh_token,
            };
            
            Ok(Json(response))
        }
        Err(UserServiceError::TokenError(msg)) => {
            Err((StatusCode::UNAUTHORIZED, msg))
        }
        Err(e) => {
            tracing::error!("刷新令牌错误: {:?}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, "服务器内部错误".to_string()))
        }
    }
}

/// 登出接口
/// 使刷新令牌失效
pub async fn logout(
    State(pool): State<sqlx::mysql::MySqlPool>,
    Json(payload): Json<LogoutRequest>
) -> Result<Json<()>, (StatusCode, String)> {
    let user_service = UserService::new(pool);
    
    match user_service.logout(&payload).await {
        Ok(_) => {
            Ok(Json(()))
        }
        Err(e) => {
            tracing::error!("登出错误: {:?}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, "服务器内部错误".to_string()))
        }
    }
}