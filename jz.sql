--创建数据库
CREATE DATABASE jz_db;

-- 创建用户（适用于MySQL 5.7及以上版本）

CREATE USER 'jz_db' @'localhost' IDENTIFIED BY 'cXMYpa2fwkFx3447';

-- 授予用户权限（例如授予所有权限）
GRANT ALL PRIVILEGES ON jz_db.* TO 'jz_db' @'localhost';

-- 或者只授予特定权限
-- GRANT SELECT, INSERT, UPDATE, DELETE ON database_name.* TO 'new_user'@'localhost';

-- 刷新权限
FLUSH PRIVILEGES;
-- jz_db:cXMYpa2fwkFx3447@127.0.0.1:3306/jz_db

use jz_db;
-- ============================================================
-- 1. 用户相关表
-- ============================================================

CREATE TABLE users (
    user_id INT PRIMARY KEY AUTO_INCREMENT COMMENT '用户ID',
    username VARCHAR(50) UNIQUE NOT NULL COMMENT '用户名',
    password_hash VARCHAR(255) NOT NULL COMMENT '密码哈希',
    email VARCHAR(100) UNIQUE NOT NULL COMMENT '邮箱',
    phone VARCHAR(20) UNIQUE NOT NULL COMMENT '手机号',
    user_type ENUM('customer', 'worker', 'admin') DEFAULT 'customer' COMMENT '用户类型',
    avatar_url VARCHAR(500) COMMENT '头像URL',
    real_name VARCHAR(50) COMMENT '真实姓名',
    is_verified BOOLEAN DEFAULT FALSE COMMENT '是否实名认证',
    balance DECIMAL(10, 2) DEFAULT 0.00 COMMENT '账户余额',
    status ENUM(
        'active',
        'inactive',
        'banned'
    ) DEFAULT 'active' COMMENT '账户状态',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    INDEX idx_phone_email (phone, email),
    INDEX idx_status (status)
) COMMENT = '用户主表';

CREATE TABLE user_addresses (
    address_id INT PRIMARY KEY AUTO_INCREMENT COMMENT '地址ID',
    user_id INT NOT NULL COMMENT '用户ID',
    contact_name VARCHAR(50) NOT NULL COMMENT '联系人',
    contact_phone VARCHAR(20) NOT NULL COMMENT '联系电话',
    province VARCHAR(50) NOT NULL COMMENT '省份',
    city VARCHAR(50) NOT NULL COMMENT '城市',
    district VARCHAR(50) NOT NULL COMMENT '区县',
    street_address VARCHAR(200) NOT NULL COMMENT '详细地址',
    is_default BOOLEAN DEFAULT FALSE COMMENT '是否默认',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users (user_id) ON DELETE CASCADE,
    INDEX idx_user (user_id),
    INDEX idx_user_default (user_id, is_default)
) COMMENT = '用户地址表';

-- ============================================================
-- 2. 服务分类与服务
-- ============================================================

CREATE TABLE service_categories (
    category_id INT PRIMARY KEY AUTO_INCREMENT COMMENT '分类ID',
    category_name VARCHAR(50) NOT NULL COMMENT '分类名称',
    parent_id INT NULL COMMENT '父分类ID',
    icon_url VARCHAR(500) COMMENT '图标URL',
    is_active BOOLEAN DEFAULT TRUE COMMENT '是否启用',
    sort_order INT DEFAULT 0 COMMENT '排序',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (parent_id) REFERENCES service_categories (category_id),
    INDEX idx_parent (parent_id)
) COMMENT = '服务分类表';

CREATE TABLE services (
    service_id INT PRIMARY KEY AUTO_INCREMENT COMMENT '服务ID',
    category_id INT NOT NULL COMMENT '分类ID',
    service_name VARCHAR(100) NOT NULL COMMENT '服务名称',
    description TEXT COMMENT '服务描述',
    base_price DECIMAL(8, 2) NOT NULL COMMENT '基础价格',
    unit ENUM(
        'hour',
        'square_meter',
        'item',
        'fixed'
    ) NOT NULL COMMENT '计价单位',
    min_duration INT DEFAULT 1 COMMENT '最小时长/数量',
    max_duration INT DEFAULT 8 COMMENT '最大时长/数量',
    is_active BOOLEAN DEFAULT TRUE COMMENT '是否启用',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (category_id) REFERENCES service_categories (category_id),
    INDEX idx_category (category_id),
    INDEX idx_active (is_active)
) COMMENT = '服务项目表';

CREATE TABLE service_addons (
    addon_id INT PRIMARY KEY AUTO_INCREMENT COMMENT '附加项ID',
    service_id INT NOT NULL COMMENT '服务ID',
    addon_name VARCHAR(100) NOT NULL COMMENT '附加项名称',
    addon_price DECIMAL(8, 2) NOT NULL COMMENT '附加项价格',
    is_active BOOLEAN DEFAULT TRUE COMMENT '是否启用',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (service_id) REFERENCES services (service_id) ON DELETE CASCADE,
    INDEX idx_service (service_id)
) COMMENT = '服务附加项表';

-- ============================================================
-- 3. 工作者相关表
-- ============================================================

CREATE TABLE worker_profiles (
    worker_id INT PRIMARY KEY COMMENT '服务人员ID',
    service_category_id INT NOT NULL COMMENT '服务分类ID',
    hourly_rate DECIMAL(8, 2) NOT NULL COMMENT '时薪',
    bio TEXT COMMENT '个人简介',
    skills JSON COMMENT '技能标签',
    service_area JSON COMMENT '服务区域',
    total_orders INT DEFAULT 0 COMMENT '总订单数',
    completed_orders INT DEFAULT 0 COMMENT '完成订单数',
    avg_rating DECIMAL(3, 2) DEFAULT 0.00 COMMENT '平均评分',
    is_available BOOLEAN DEFAULT TRUE COMMENT '是否可接单',
    max_daily_orders INT DEFAULT 3 COMMENT '每日最大接单数',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    FOREIGN KEY (worker_id) REFERENCES users (user_id) ON DELETE CASCADE,
    FOREIGN KEY (service_category_id) REFERENCES service_categories (category_id),
    INDEX idx_category_rating (
        service_category_id,
        avg_rating DESC
    ),
    INDEX idx_available (is_available)
) COMMENT = '服务人员详情表';

-- ============================================================
-- 4. 优惠券基础表
-- ============================================================

CREATE TABLE coupons (
    coupon_id INT PRIMARY KEY AUTO_INCREMENT COMMENT '优惠券ID',
    coupon_code VARCHAR(20) UNIQUE NOT NULL COMMENT '优惠券码',
    coupon_name VARCHAR(100) NOT NULL COMMENT '优惠券名称',
    discount_type ENUM(
        'percentage',
        'fixed',
        'service'
    ) NOT NULL COMMENT '折扣类型',
    discount_value DECIMAL(10, 2) NOT NULL COMMENT '折扣值',
    min_order_amount DECIMAL(10, 2) DEFAULT 0.00 COMMENT '最低订单金额',
    applicable_services JSON COMMENT '适用服务',
    valid_from DATE NOT NULL COMMENT '有效期开始',
    valid_until DATE NOT NULL COMMENT '有效期截止',
    usage_limit INT DEFAULT 1 COMMENT '使用次数限制',
    used_count INT DEFAULT 0 COMMENT '已使用次数',
    is_active BOOLEAN DEFAULT TRUE COMMENT '是否启用',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    INDEX idx_code_validity (
        coupon_code,
        valid_until,
        is_active
    )
) COMMENT = '优惠券表';

-- ============================================================
-- 5. 订单主表（依赖多表）
-- ============================================================

CREATE TABLE orders (
    order_id VARCHAR(20) PRIMARY KEY COMMENT '订单号',
    customer_id INT NOT NULL COMMENT '客户ID',
    worker_id INT NULL COMMENT '服务人员ID',
    address_id INT NOT NULL COMMENT '地址ID',
    service_id INT NOT NULL COMMENT '服务ID',
    coupon_id INT NULL COMMENT '优惠券ID',
    service_date DATE NOT NULL COMMENT '服务日期',
    time_slot ENUM(
        'morning',
        'afternoon',
        'evening',
        'full_day'
    ) NOT NULL COMMENT '服务时段',
    duration DECIMAL(5, 1) NOT NULL COMMENT '服务时长',
    unit_price DECIMAL(8, 2) NOT NULL COMMENT '单价',
    subtotal DECIMAL(10, 2) NOT NULL COMMENT '小计金额',
    discount_amount DECIMAL(8, 2) DEFAULT 0.00 COMMENT '优惠金额',
    total_amount DECIMAL(10, 2) NOT NULL COMMENT '总金额',
    payment_status ENUM('pending', 'paid', 'refunded') DEFAULT 'pending' COMMENT '支付状态',
    order_status ENUM(
        'pending',
        'confirmed',
        'assigned',
        'ongoing',
        'completed',
        'cancelled'
    ) DEFAULT 'pending' COMMENT '订单状态',
    special_instructions TEXT COMMENT '特殊要求',
    cancellation_reason TEXT COMMENT '取消原因',
    scheduled_start_time DATETIME NULL COMMENT '计划开始时间',
    actual_start_time DATETIME NULL COMMENT '实际开始时间',
    actual_end_time DATETIME NULL COMMENT '实际结束时间',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    FOREIGN KEY (customer_id) REFERENCES users (user_id),
    FOREIGN KEY (worker_id) REFERENCES users (user_id),
    FOREIGN KEY (address_id) REFERENCES user_addresses (address_id),
    FOREIGN KEY (service_id) REFERENCES services (service_id),
    FOREIGN KEY (coupon_id) REFERENCES coupons (coupon_id),
    INDEX idx_customer_status (customer_id, order_status),
    INDEX idx_worker_status (worker_id, order_status),
    INDEX idx_date_status (service_date, order_status)
) COMMENT = '订单主表';

-- ============================================================
-- 6. 订单附加项
-- ============================================================

CREATE TABLE order_addons (
    order_addon_id INT PRIMARY KEY AUTO_INCREMENT COMMENT '订单附加项ID',
    order_id VARCHAR(20) NOT NULL COMMENT '订单ID',
    addon_id INT NOT NULL COMMENT '附加项ID',
    quantity INT DEFAULT 1 COMMENT '数量',
    unit_price DECIMAL(8, 2) NOT NULL COMMENT '单价',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (order_id) REFERENCES orders (order_id) ON DELETE CASCADE,
    FOREIGN KEY (addon_id) REFERENCES service_addons (addon_id),
    INDEX idx_order (order_id)
) COMMENT = '订单附加项表';

-- ============================================================
-- 7. 工作者日程（需要 orders）
-- ============================================================

CREATE TABLE worker_schedules (
    schedule_id INT PRIMARY KEY AUTO_INCREMENT COMMENT '日程ID',
    worker_id INT NOT NULL COMMENT '服务人员ID',
    schedule_date DATE NOT NULL COMMENT '排班日期',
    time_slot ENUM(
        'morning',
        'afternoon',
        'evening',
        'full_day'
    ) NOT NULL COMMENT '时间段',
    status ENUM(
        'available',
        'booked',
        'unavailable'
    ) DEFAULT 'available' COMMENT '状态',
    order_id VARCHAR(20) NULL COMMENT '关联订单ID',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (worker_id) REFERENCES users (user_id) ON DELETE CASCADE,
    FOREIGN KEY (order_id) REFERENCES orders (order_id),
    UNIQUE KEY uk_worker_time (
        worker_id,
        schedule_date,
        time_slot
    ),
    INDEX idx_worker_date (worker_id, schedule_date)
) COMMENT = '服务人员日程表';

-- ============================================================
-- 8. 用户优惠券
-- ============================================================

CREATE TABLE user_coupons (
    user_coupon_id INT PRIMARY KEY AUTO_INCREMENT COMMENT '用户优惠券ID',
    user_id INT NOT NULL COMMENT '用户ID',
    coupon_id INT NOT NULL COMMENT '优惠券ID',
    order_id VARCHAR(20) NULL COMMENT '订单ID',
    is_used BOOLEAN DEFAULT FALSE COMMENT '是否已使用',
    used_at DATETIME NULL COMMENT '使用时间',
    expires_at DATE NOT NULL COMMENT '过期时间',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users (user_id),
    FOREIGN KEY (coupon_id) REFERENCES coupons (coupon_id),
    FOREIGN KEY (order_id) REFERENCES orders (order_id),
    UNIQUE KEY uk_user_coupon_order (user_id, coupon_id, order_id),
    INDEX idx_user_unused (user_id, is_used, expires_at)
) COMMENT = '用户优惠券表';

-- ============================================================
-- 9. 评价与投诉
-- ============================================================

CREATE TABLE reviews (
    review_id INT PRIMARY KEY AUTO_INCREMENT COMMENT '评价ID',
    order_id VARCHAR(20) UNIQUE NOT NULL COMMENT '订单ID',
    customer_id INT NOT NULL COMMENT '客户ID',
    worker_id INT NOT NULL COMMENT '服务人员ID',
    rating TINYINT NOT NULL COMMENT '评分(1-5)',
    service_rating TINYINT COMMENT '服务评分',
    punctuality_rating TINYINT COMMENT '守时评分',
    review_text TEXT COMMENT '评价内容',
    is_anonymous BOOLEAN DEFAULT FALSE COMMENT '是否匿名',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (order_id) REFERENCES orders (order_id),
    FOREIGN KEY (customer_id) REFERENCES users (user_id),
    FOREIGN KEY (worker_id) REFERENCES users (user_id),
    INDEX idx_worker_rating (worker_id, rating DESC),
    INDEX idx_order (order_id)
) COMMENT = '评价表';

CREATE TABLE complaints (
    complaint_id INT PRIMARY KEY AUTO_INCREMENT COMMENT '投诉ID',
    order_id VARCHAR(20) NOT NULL COMMENT '订单ID',
    complainant_id INT NOT NULL COMMENT '投诉人ID',
    target_id INT NOT NULL COMMENT '被投诉人ID',
    complaint_type ENUM(
        'service',
        'punctuality',
        'attitude',
        'other'
    ) NOT NULL COMMENT '投诉类型',
    complaint_text TEXT NOT NULL COMMENT '投诉内容',
    status ENUM(
        'pending',
        'investigating',
        'resolved',
        'rejected'
    ) DEFAULT 'pending' COMMENT '处理状态',
    resolved_at DATETIME NULL COMMENT '处理时间',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (order_id) REFERENCES orders (order_id),
    FOREIGN KEY (complainant_id) REFERENCES users (user_id),
    FOREIGN KEY (target_id) REFERENCES users (user_id),
    INDEX idx_status (status),
    INDEX idx_complainant (complainant_id)
) COMMENT = '投诉表';

-- ============================================================
-- 10. 支付表
-- ============================================================

CREATE TABLE payments (
    payment_id VARCHAR(30) PRIMARY KEY COMMENT '支付ID',
    order_id VARCHAR(20) NOT NULL COMMENT '订单ID',
    user_id INT NOT NULL COMMENT '用户ID',
    payment_method ENUM(
        'wechat',
        'alipay',
        'balance',
        'card'
    ) NOT NULL COMMENT '支付方式',
    payment_amount DECIMAL(10, 2) NOT NULL COMMENT '支付金额',
    payment_status ENUM(
        'pending',
        'processing',
        'success',
        'failed'
    ) DEFAULT 'pending' COMMENT '支付状态',
    thirdparty_trade_no VARCHAR(100) COMMENT '第三方交易号',
    payment_time DATETIME NULL COMMENT '支付时间',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (order_id) REFERENCES orders (order_id),
    FOREIGN KEY (user_id) REFERENCES users (user_id),
    INDEX idx_order_status (order_id, payment_status),
    INDEX idx_user_time (user_id, payment_time DESC)
) COMMENT = '支付记录表';

-- ============================================================
-- 11. 系统配置与通知
-- ============================================================

CREATE TABLE system_settings (
    setting_key VARCHAR(50) PRIMARY KEY COMMENT '配置键',
    setting_value TEXT COMMENT '配置值',
    category VARCHAR(50) COMMENT '分类',
    description TEXT COMMENT '描述',
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
) COMMENT = '系统配置表';

CREATE TABLE notifications (
    notification_id INT PRIMARY KEY AUTO_INCREMENT COMMENT '消息ID',
    user_id INT NOT NULL COMMENT '用户ID',
    notification_type ENUM(
        'order',
        'system',
        'promotion'
    ) NOT NULL COMMENT '消息类型',
    title VARCHAR(200) NOT NULL COMMENT '标题',
    content TEXT NOT NULL COMMENT '内容',
    related_id VARCHAR(50) COMMENT '关联业务ID',
    is_read BOOLEAN DEFAULT FALSE COMMENT '是否已读',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users (user_id),
    INDEX idx_user_unread (user_id, is_read),
    INDEX idx_created (created_at DESC)
) COMMENT = '消息通知表';