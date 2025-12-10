<template>
	<view class="container">
		<!-- 用户信息头部 -->
		<view class="user-header">
			<image class="user-bg" src="/static/user-bg.jpg"></image>
			<view class="user-info">
				<image class="avatar" :src="user.avatar_url || '/static/avatar.jpg'"></image>
				<view class="user-details">
					<text class="username" @click="goToLogin">{{ user.username || '请登录' }}</text>
					<text class="user-phone">{{ formatPhoneNumber(user.phone) || '未绑定手机号' }}</text>
				</view>
			</view>
		</view>
		
		<!-- 未登录时显示提示 -->
		<view v-if="!isLoggedIn" class="login-prompt-section">
			<view class="prompt-box">
				<text class="prompt-text">请登录以查看您的个人信息和订单</text>
				<button class="login-button" @click="goToLogin">立即登录</button>
			</view>
		</view>
		
		<!-- 登录后显示的内容 -->
		<view v-else>
			<!-- 微信信息获取 -->
			<view v-if="isWechat && !hasWechatInfo" class="section">
				<view class="function-item" @click="getWechatInfo">
					<image class="function-icon" src="/static/wechat-icon.png"></image>
					<text class="function-text">获取微信信息</text>
					<text class="arrow-icon">></text>
				</view>
			</view>
			
			<!-- 我的订单 -->
			<view class="section">
				<view class="section-header" @click="goToOrders">
					<text class="section-title">我的订单</text>
					<view class="arrow">
						<text>全部订单</text>
						<text class="arrow-icon">></text>
					</view>
				</view>
				
				<view class="order-stats">
					<view class="stat-item" @click="goToOrdersWithStatus(1)">
						<image class="stat-icon" src="/static/pending-payment.png"></image>
						<text class="stat-label">待付款</text>
					</view>
					<view class="stat-item" @click="goToOrdersWithStatus(2)">
						<image class="stat-icon" src="/static/pending-service.png"></image>
						<text class="stat-label">待服务</text>
					</view>
					<view class="stat-item" @click="goToOrdersWithStatus(3)">
						<image class="stat-icon" src="/static/in-service.png"></image>
						<text class="stat-label">服务中</text>
					</view>
					<view class="stat-item" @click="goToOrdersWithStatus(4)">
						<image class="stat-icon" src="/static/completed.png"></image>
						<text class="stat-label">已完成</text>
					</view>
				</view>
			</view>
			
			<!-- 其他功能 -->
			<view class="section">
				<view class="function-item">
					<image class="function-icon" src="/static/address.png"></image>
					<text class="function-text">地址管理</text>
					<text class="arrow-icon">></text>
				</view>
				<view class="function-item">
					<image class="function-icon" src="/static/coupon.png"></image>
					<text class="function-text">优惠券</text>
					<text class="arrow-icon">></text>
				</view>
				<view class="function-item">
					<image class="function-icon" src="/static/collection.png"></image>
					<text class="function-text">我的收藏</text>
					<text class="arrow-icon">></text>
				</view>
			</view>
			
			<view class="section">
				<view class="function-item" @click="refreshUserInfo">
					<image class="function-icon" src="/static/refresh.png"></image>
					<text class="function-text">刷新信息</text>
					<text class="arrow-icon">></text>
				</view>
				<view class="function-item">
					<image class="function-icon" src="/static/customer-service.png"></image>
					<text class="function-text">联系客服</text>
					<text class="arrow-icon">></text>
				</view>
				<view class="function-item" @click="logout">
					<image class="function-icon" src="/static/settings.png"></image>
					<text class="function-text">退出登录</text>
					<text class="arrow-icon">></text>
				</view>
			</view>
		</view>
	</view>
</template>

<script>
	import { userApi } from '../../api/api.js';
	import { getWechatUserInfo, getSystemUserInfo } from '../../utils/user.js';
	
	export default {
		data() {
			return {
				user: {},
				isLoggedIn: false,
				isWechat: false,
				hasWechatInfo: false
			}
		},
		onLoad() {
			this.loadUserInfo();
			
			// 检测是否为微信环境
			// #ifdef MP-WEIXIN
			this.isWechat = true;
			// #endif
		},
		onShow() {
			// 页面显示时也加载用户信息，确保信息是最新的
			this.loadUserInfo();
		},
		methods: {
			loadUserInfo() {
				const user = getSystemUserInfo();
				if (user) {
					this.user = user;
					this.isLoggedIn = true;
				} else {
					this.isLoggedIn = false;
				}
			},
			
			async getWechatInfo() {
				try {
					const wechatUserInfo = await getWechatUserInfo();
					this.hasWechatInfo = true;
					
					// 可以选择将微信信息与系统用户信息合并
					this.user = {
						...this.user,
						...wechatUserInfo
					};
					
					// 更新本地存储
					uni.setStorageSync('user', JSON.stringify(this.user));
					
					uni.showToast({
						title: '获取成功',
						icon: 'success'
					});
				} catch (err) {
					console.error('获取微信用户信息失败:', err);
					uni.showToast({
						title: err.message.includes('fail') ? '获取失败' : err.message,
						icon: 'none'
					});
				}
			},
			
			async refreshUserInfo() {
				try {
					const userInfo = await userApi.getUserProfile();
					this.user = userInfo;
					
					// 更新本地存储
					uni.setStorageSync('user', JSON.stringify(userInfo));
					
					uni.showToast({
						title: '信息已更新',
						icon: 'success'
					});
				} catch (err) {
					console.error('获取用户信息失败:', err);
					uni.showToast({
						title: '获取信息失败',
						icon: 'none'
					});
				}
			},
			
			goToLogin() {
				uni.navigateTo({
					url: '/pages/auth/auth'
				});
			},
			
			formatPhoneNumber(phone) {
				if (!phone) return '';
				return phone.replace(/(\d{3})\d{4}(\d{4})/, '$1****$2');
			},
			
			goToOrders() {
				uni.navigateTo({
					url: '/pages/orders/orders'
				})
			},
			
			goToOrdersWithStatus(status) {
				uni.navigateTo({
					url: `/pages/orders/orders?status=${status}`
				})
			},
			
			async logout() {
				uni.showModal({
					title: '退出登录',
					content: '确定要退出登录吗？',
					success: async (res) => {
						if (res.confirm) {
							try {
								await userApi.logout();
								
								this.isLoggedIn = false;
								this.user = {};
								
								uni.showToast({
									title: '已退出登录',
									icon: 'success'
								});
							} catch (err) {
								console.error('登出失败:', err);
								uni.showToast({
									title: '登出失败',
									icon: 'none'
								});
							}
						}
					}
				});
			}
		}
	}
</script>

<style>
	.container {
		background-color: #f5f5f5;
		min-height: 100vh;
	}
	
	.user-header {
		position: relative;
		height: 300rpx;
		overflow: hidden;
	}
	
	.user-bg {
		width: 100%;
		height: 100%;
	}
	
	.user-info {
		position: absolute;
		bottom: 30rpx;
		left: 30rpx;
		display: flex;
		align-items: center;
	}
	
	.avatar {
		width: 120rpx;
		height: 120rpx;
		border-radius: 50%;
		border: 5rpx solid rgba(255, 255, 255, 0.5);
		margin-right: 20rpx;
	}
	
	.user-details {
		display: flex;
		flex-direction: column;
	}
	
	.username {
		font-size: 36rpx;
		font-weight: bold;
		color: #ffffff;
		margin-bottom: 10rpx;
	}
	
	.user-phone {
		font-size: 28rpx;
		color: rgba(255, 255, 255, 0.9);
	}
	
	/* 添加可点击样式 */
	.username:active {
		opacity: 0.8;
	}
	
	.login-prompt-section {
		margin: 20rpx;
		padding: 40rpx 20rpx;
		background-color: #ffffff;
		border-radius: 20rpx;
		text-align: center;
		box-shadow: 0 5rpx 15rpx rgba(0,0,0,0.05);
	}
	
	.prompt-text {
		font-size: 30rpx;
		color: #666;
		margin-bottom: 30rpx;
		display: block;
	}
	
	.login-button {
		background-color: #00bfff;
		color: white;
		border: none;
		padding: 30rpx 60rpx;
		font-size: 32rpx;
		border-radius: 10rpx;
	}
	
	.section {
		background-color: #ffffff;
		margin: 20rpx;
		border-radius: 20rpx;
		overflow: hidden;
		box-shadow: 0 5rpx 15rpx rgba(0,0,0,0.05);
	}
	
	.section-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 30rpx;
		border-bottom: 1rpx solid #f5f5f5;
	}
	
	.section-title {
		font-size: 32rpx;
		font-weight: bold;
		color: #333333;
	}
	
	.arrow {
		display: flex;
		align-items: center;
		font-size: 26rpx;
		color: #999999;
	}
	
	.arrow-icon {
		margin-left: 10rpx;
	}
	
	.order-stats {
		display: flex;
		justify-content: space-around;
		padding: 30rpx 0;
	}
	
	.stat-item {
		display: flex;
		flex-direction: column;
		align-items: center;
	}
	
	.stat-icon {
		width: 60rpx;
		height: 60rpx;
		margin-bottom: 15rpx;
	}
	
	.stat-label {
		font-size: 26rpx;
		color: #666666;
	}
	
	.function-item {
		display: flex;
		align-items: center;
		padding: 30rpx;
		border-bottom: 1rpx solid #f5f5f5;
	}
	
	.function-item:last-child {
		border-bottom: none;
	}
	
	.function-icon {
		width: 50rpx;
		height: 50rpx;
		margin-right: 20rpx;
	}
	
	.function-text {
		flex: 1;
		font-size: 30rpx;
		color: #333333;
	}
</style>