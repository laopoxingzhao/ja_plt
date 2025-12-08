# 家政服务后端 API

这是一个使用 Rust 编写的家政服务后端 API 项目。

## 功能特性

- 用户注册和登录（JWT 认证）
- 服务管理
- 订单处理
- 支付和优惠券系统
- 通知和评价系统

## 技术栈

- Rust 2024 edition
- Axum web 框架
- SQLx 数据库工具
- MySQL/SQLite 数据库
- Tokio 异步运行时
- Serde 序列化
- Argon2 密码哈希
- jsonwebtoken JWT 处理

## 如何运行测试

本项目包含多种类型的测试：

### 单元测试

单元测试位于各个模块文件中，可以使用以下命令运行：

```bash
cargo test
```

这将运行所有的单元测试。

### 集成测试

集成测试位于 [tests](file:///e:/Rust/frontend/tests) 目录中，同样可以通过以下命令运行：

```bash
cargo test
```

注意：某些集成测试可能需要数据库连接才能正常工作。

### 运行特定测试

只运行单元测试：
```bash
cargo test --lib
```

只运行集成测试：
```bash
cargo test --test integration_tests
```

运行特定测试函数：
```bash
cargo test test_jwt_generation
```

## 项目结构

```
src/
├── config/          # 配置相关
├── database/        # 数据库连接
├── handler/         # 请求处理器
├── models/          # 数据模型
├── repositories/    # 数据访问层
├── services/        # 业务逻辑层
├── utils/           # 工具函数
└── main.rs          # 应用入口
tests/
├── integration_tests.rs  # 集成测试
├── user_tests.rs        # 用户相关测试
├── auth_tests.rs        # 认证相关测试
└── service_tests.rs     # 服务相关测试
```

## 环境配置

在运行测试之前，请确保设置了必要的环境变量。可以创建一个 `.env` 文件：

```env
DATABASE_URL=mysql://user:password@localhost/database_name
JWT_SECRET=your_jwt_secret_here
```

## 测试策略

1. **单元测试** - 测试单个函数或方法的功能
2. **集成测试** - 测试多个组件协同工作的功能
3. **端到端测试** - 模拟真实用户场景的完整流程测试

目前项目包含了基础的测试框架，可以根据需要扩展更多具体的测试用例。