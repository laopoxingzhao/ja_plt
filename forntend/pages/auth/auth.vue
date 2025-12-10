<template>
	<view class="container">
		<view class="header">
			<text class="title">{{ isLoggedIn ? '用户中心' : '用户登录' }}</text>
		</view>
		
		<AuthComponent 
			v-if="!isLoggedIn" 
			@login-success="onLoginSuccess" 
			@register-success="onRegisterSuccess" 
			@logout-success="onLogoutSuccess"
		/>
		
		<view v-else class="user-info">
			<view class="avatar">
				<image :src="user.avatar_url || '/static/default-avatar.png'" class="avatar-img" />
			</view>
			<view class="user-details">
				<text class="detail-text username">{{ user.username }}</text>
				<text class="detail-text email">{{ user.email }}</text>
				<text class="detail-text phone">{{ user.phone }}</text>
				<text class="detail-text balance">余额: ¥{{ user.balance.toFixed(2) }}</text>
			</view>
			<button class="logout-btn" @click="handleLogout">
				<text class="logout-text">退出登录</text>
			</button>
		</view>
		
		<view class="footer">
			<navigator url="/pages/index/index" class="link">返回首页</navigator>
		</view>
	</view>
</template>

<script>
	import AuthComponent from '@/components/AuthComponent.vue';
	import { userApi } from '@/api/api.js';
	
	export default {
		components: {
			AuthComponent
		},
		data() {
			return {
				isLoggedIn: false,
				user: {}
			}
		},
		onLoad() {
			// 页面加载时检查登录状态
			this.checkLoginStatus();
		},
		methods: {
			checkLoginStatus() {
				const token = uni.getStorageSync('token');
				const userStr = uni.getStorageSync('user');
				
				if (token && userStr) {
					try {
						this.user = JSON.parse(userStr);
						this.isLoggedIn = true;
					} catch (e) {
						console.error('解析用户信息失败:', e);
						this.isLoggedIn = false;
						this.user = {};
					}
				} else {
					this.isLoggedIn = false;
					this.user = {};
				}
			},
			
			onLoginSuccess(user) {
				this.user = user;
				this.isLoggedIn = true;
				
				uni.showToast({
					title: '登录成功',
					icon: 'success'
				});
				
				// 返回上一页或跳转到用户中心
				setTimeout(() => {
					uni.switchTab({
						url: '/pages/index/index'
					});
				}, 1000);
			},
			
			onRegisterSuccess(username) {
				uni.showToast({
					title: '注册成功',
					icon: 'success'
				});
			},
			
			onLogoutSuccess() {
				this.isLoggedIn = false;
				this.user = {};
				
				uni.showToast({
					title: '已退出登录',
					icon: 'success'
				});
			},
			
			handleLogout() {
				// 调用子组件的登出方法
				// 这里直接调用API
				userApi.logout().then(() => {
					uni.removeStorageSync('token');
					uni.removeStorageSync('user');
					uni.removeStorageSync('refreshToken');
					
					this.isLoggedIn = false;
					this.user = {};
					
					uni.showToast({
						title: '已退出登录',
						icon: 'success'
					});
				}).catch(err => {
					uni.showToast({
						title: err.message || '登出失败',
						icon: 'none'
					});
				});
			}
		}
	}
</script>

<style>
	.container {
		background: linear-gradient(135deg, #f5f7fa 0%, #c3cfe2 100%);
		min-height: 100vh;
		padding: 20rpx;
	}

	.header {
		text-align: center;
		padding: 60rpx 0 40rpx;
	}

	.title {
		font-size: 48rpx;
		font-weight: bold;
		color: #333333;
		text-shadow: 2rpx 2rpx 4rpx rgba(0, 0, 0, 0.1);
	}

	.user-info {
		background-color: #ffffff;
		border-radius: 20rpx;
		padding: 60rpx 40rpx;
		text-align: center;
		margin-bottom: 30rpx;
		box-shadow: 0 10rpx 30rpx rgba(0, 0, 0, 0.05);
	}

	.avatar {
		margin-bottom: 40rpx;
	}

	.avatar-img {
		width: 160rpx;
		height: 160rpx;
		border-radius: 50%;
		border: 6rpx solid #f0f0f0;
		box-shadow: 0 10rpx 20rpx rgba(0, 0, 0, 0.1);
	}

	.user-details {
		display: flex;
		flex-direction: column;
		gap: 30rpx;
		margin-bottom: 50rpx;
	}

	.detail-text {
		font-size: 28rpx;
		color: #333333;
	}

	.username {
		font-size: 40rpx;
		font-weight: bold;
	}
```

e:\Rust\frontend\forntend\pages\auth\auth.vue
```vue
<<<<<<< SEARCH
	.logout-btn {
		background: linear-gradient(90deg, #ff4d4f, #ff1a1a);
		color: white;
		border: none;
		padding: 25rpx;
		border-radius: 12rpx;
		font-size: 32rpx;
		font-weight: 500;
		letter-spacing: 2rpx;
		box-shadow: 0 10rpx 20rpx rgba(255, 77, 79, 0.3);
		transition: all 0.3s ease;
	}
	.logout-btn {
		background: linear-gradient(90deg, #ff4d4f, #ff1a1a);
		border: none;
		padding: 25rpx;
		border-radius: 12rpx;
		font-size: 32rpx;
		font-weight: 500;
		letter-spacing: 2rpx;
		box-shadow: 0 10rpx 20rpx rgba(255, 77, 79, 0.3);
		transition: all 0.3s ease;
	}
	
	.logout-text {
		color: #ffffff;
	}

	.email::before {
		content: "📧 ";
		margin-right: 10rpx;
	}

	.phone::before {
		content: "📱 ";
		margin-right: 10rpx;
	}

	.balance::before {
		content: "💰 ";
		margin-right: 10rpx;
	}

	.logout-btn {
		background: linear-gradient(90deg, #ff4d4f, #ff1a1a);
		color: white;
		border: none;
		padding: 25rpx;
		border-radius: 12rpx;
		font-size: 32rpx;
		font-weight: 500;
		letter-spacing: 2rpx;
		box-shadow: 0 10rpx 20rpx rgba(255, 77, 79, 0.3);
		transition: all 0.3s ease;
	}

	.logout-btn:active {
		transform: scale(0.98);
		box-shadow: 0 5rpx 10rpx rgba(255, 77, 79, 0.2);
	}

	.footer {
		text-align: center;
		padding: 40rpx 0;
	}

	.link {
		color: #00bfff;
		font-size: 28rpx;
		text-decoration: underline;
	}
</style>