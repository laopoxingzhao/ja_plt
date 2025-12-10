# 家政服务前端项目

## 项目简介

这是一个基于uni-app开发的家政服务前端项目，使用Vue.js框架，可以编译到多个平台包括H5、微信小程序等。

## 技术栈

- uni-app
- Vue.js
- Uni.request (内置网络请求)

## 目录结构

```
.
├── api/              # API接口封装
├── components/       # 公共组件
├── mock/             # 已废弃（使用api.js中的简单mock实现）
├── pages/            # 页面文件
├── static/           # 静态资源
├── App.vue           # 应用配置
├── main.js           # 入口文件
├── pages.json        # 页面配置
└── uni.scss          # 全局样式
```

## 环境配置

项目支持两种环境：
- 开发环境 (.env.development)
- 生产环境 (.env.production)

## 安装依赖

```bash
npm install
```

## 运行项目

### 运行到H5平台

```bash
npm run dev:h5
```

### 运行到微信小程序

```bash
npm run dev:mp-weixin
```

## 构建项目

### 构建H5版本

```bash
npm run build:h5
```

### 构建微信小程序版本

```bash
npm run build:mp-weixin
```

## Mock数据

在开发环境中，项目使用简单的内联Mock数据提供模拟数据，无需依赖后端服务或外部库。

## 注意事项

1. 生产环境需要配置真实的API地址
2. 确保在生产环境中关闭Mock功能
3. 项目使用token进行身份验证，请妥善保管用户凭证