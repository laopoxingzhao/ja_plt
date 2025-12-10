//! 用户数据访问层
//!
//! 负责用户相关的数据库操作

use sqlx::{mysql::MySqlPool, Row};
use crate::models::user::User;

pub struct UserRepository {
    pool: MySqlPool,
}

impl UserRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }

    /// 获取所有用户列表
    pub async fn list_users(&self) -> Result<Vec<User>, sqlx::Error> {
        let users_result = sqlx::query(
            "SELECT user_id, username, password_hash, email, phone, user_type, avatar_url, \
            real_name, is_verified, balance, status, created_at, updated_at \
            FROM users"
        )
        .fetch_all(&self.pool)
        .await;

        match users_result {
            Ok(rows) => {
                let users: Vec<User> = rows.into_iter().map(|row| {
                    User {
                        user_id: row.get("user_id"),
                        username: row.get("username"),
                        password_hash: row.get("password_hash"),
                        email: row.get("email"),
                        phone: row.get("phone"),
                        user_type: row.get("user_type"),
                        avatar_url: row.get("avatar_url"),
                        real_name: row.get("real_name"),
                        is_verified: row.get("is_verified"),
                        balance: row.get("balance"),
                        status: row.get("status"),
                        created_at: row.get("created_at"),
                        updated_at: row.get("updated_at"),
                    }
                }).collect();
                Ok(users)
            }
            Err(e) => Err(e),
        }
    }

    /// 根据ID查找用户
    pub async fn find_by_id(&self, id: i32) -> Result<User, sqlx::Error> {
        let user_result = sqlx::query(
            "SELECT user_id, username, password_hash, email, phone, user_type, avatar_url, \
            real_name, is_verified, balance, status, created_at, updated_at \
            FROM users WHERE user_id = ?"
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await;

        match user_result {
            Ok(row) => {
                let user = User {
                    user_id: row.get("user_id"),
                    username: row.get("username"),
                    password_hash: row.get("password_hash"),
                    email: row.get("email"),
                    phone: row.get("phone"),
                    user_type: row.get("user_type"),
                    avatar_url: row.get("avatar_url"),
                    real_name: row.get("real_name"),
                    is_verified: row.get("is_verified"),
                    balance: row.get("balance"),
                    status: row.get("status"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                };
                Ok(user)
            }
            Err(e) => Err(e),
        }
    }

    /// 根据用户名或邮箱查找用户
    pub async fn find_by_identifier(&self, identifier: &str) -> Result<Option<User>, sqlx::Error> {
        let user_result = sqlx::query(
            "SELECT user_id, username, password_hash, email, phone, user_type, avatar_url, \
            real_name, is_verified, balance, status, created_at, updated_at \
            FROM users WHERE username = ? OR email = ? LIMIT 1"
        )
        .bind(identifier)
        .bind(identifier)
        .fetch_optional(&self.pool)
        .await;

        match user_result {
            Ok(Some(row)) => {
                let user = User {
                    user_id: row.get("user_id"),
                    username: row.get("username"),
                    password_hash: row.get("password_hash"),
                    email: row.get("email"),
                    phone: row.get("phone"),
                    user_type: row.get("user_type"),
                    avatar_url: row.get("avatar_url"),
                    real_name: row.get("real_name"),
                    is_verified: row.get("is_verified"),
                    balance: row.get("balance"),
                    status: row.get("status"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                };
                Ok(Some(user))
            }
            Ok(None) => Ok(None),
            Err(e) => Err(e),
        }
    }

    /// 检查用户是否已存在
    pub async fn check_user_exists(&self, username: &str, email: &str, phone: &str) -> Result<bool, sqlx::Error> {
        let exists_result = sqlx::query(
            "SELECT COUNT(*) as count FROM users WHERE username = ? OR email = ? OR phone = ?"
        )
        .bind(username)
        .bind(email)
        .bind(phone)
        .fetch_one(&self.pool)
        .await;

        match exists_result {
            Ok(row) => {
                let count: i64 = row.get("count");
                Ok(count > 0)
            }
            Err(e) => Err(e),
        }
    }

    /// 创建新用户
    pub async fn create_user(&self, username: &str, hashed_password: &str, email: &str, phone: &str) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO users (username, password_hash, email, phone, user_type, status, created_at, updated_at) \
            VALUES (?, ?, ?, ?, 'customer', 'active', NOW(), NOW())"
        )
        .bind(username)
        .bind(hashed_password)
        .bind(email)
        .bind(phone)
        .execute(&self.pool)
        .await
        .map(|_| ())
    }

    /// 根据用户名查找用户
    pub async fn find_by_username(&self, username: &str) -> Result<Option<User>, sqlx::Error> {
        let user_result = sqlx::query(
            "SELECT user_id, username, password_hash, email, phone, user_type, avatar_url, \
            real_name, is_verified, balance, status, created_at, updated_at \
            FROM users WHERE username = ?"
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await;

        match user_result {
            Ok(Some(row)) => {
                let user = User {
                    user_id: row.get("user_id"),
                    username: row.get("username"),
                    password_hash: row.get("password_hash"),
                    email: row.get("email"),
                    phone: row.get("phone"),
                    user_type: row.get("user_type"),
                    avatar_url: row.get("avatar_url"),
                    real_name: row.get("real_name"),
                    is_verified: row.get("is_verified"),
                    balance: row.get("balance"),
                    status: row.get("status"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                };
                Ok(Some(user))
            }
            Ok(None) => Ok(None),
            Err(e) => Err(e),
        }
    }
}