<template>
	<view class="container">
		<!-- 服务详情头部 -->
		<view class="service-header">
			<image class="service-image" :src="service.image || '/static/service-detail.jpg'" mode="widthFix"></image>
			<view class="service-basic-info">
				<text class="service-name">{{ service.service_name }}</text>
				<text class="service-description">{{ service.description || '专业服务' }}</text>
				<view class="price-section">
					<text class="price-label">价格：</text>
					<text class="price">￥{{ service.base_price }}</text>
				</view>
			</view>
		</view>
		
		<!-- 服务详情介绍 -->
		<view class="detail-section">
			<view class="section-title">服务详情</view>
			<view class="detail-content">
				<text class="detail-text">【服务内容】</text>
				<text class="detail-text">{{ service.description || '专业服务提供' }}</text>
				<text class="detail-text">【服务时长】约{{ service.min_duration }}-{{ service.max_duration }}{{ getUnitLabel(service.unit) }}</text>
				<text class="detail-text">【计价单位】{{ getUnitLabel(service.unit) }}</text>
			</view>
		</view>
		
		<!-- 底部操作栏 -->
		<view class="bottom-bar">
			<button class="contact-button">联系客服</button>
			<button class="book-button" @click="goToBooking">立即预约</button>
		</view>
	</view>
</template>

<script>
	import { serviceApi } from '../../api/api.js';
	
	export default {
		data() {
			return {
				serviceId: null,
				service: {}
			}
		},
		onLoad(options) {
			if (options.id) {
				this.serviceId = options.id;
				this.loadServiceDetail();
			}
		},
		methods: {
			async loadServiceDetail() {
				if (!this.serviceId) return;
				
				try {
					const service = await serviceApi.getServiceById(this.serviceId);
					this.service = service;
				} catch (err) {
					console.error('获取服务详情失败:', err);
					uni.showToast({
						title: '获取服务详情失败',
						icon: 'none'
					});
				}
			},
			
			getUnitLabel(unit) {
				const unitMap = {
					'hour': '小时',
					'square_meter': '平方米',
					'item': '项',
					'fixed': '固定'
				};
				return unitMap[unit] || unit;
			},
			
			goToBooking() {
				uni.navigateTo({
					url: `/pages/booking/booking?id=${this.serviceId}`
				})
			}
		}
	}
</script>

<style>
	.container {
		background-color: #f5f5f5;
		min-height: 100vh;
		padding-bottom: 120rpx;
	}
	
	.service-header {
		background-color: #ffffff;
		margin-bottom: 20rpx;
	}
	
	.service-image {
		width: 100%;
		height: 400rpx;
	}
	
	.service-basic-info {
		padding: 30rpx;
	}
	
	.service-name {
		font-size: 36rpx;
		font-weight: bold;
		color: #333333;
		display: block;
		margin-bottom: 20rpx;
	}
	
	.service-description {
		font-size: 28rpx;
		color: #666666;
		display: block;
		margin-bottom: 30rpx;
	}
	
	.price-section {
		display: flex;
		align-items: center;
	}
	
	.price-label {
		font-size: 28rpx;
		color: #333333;
	}
	
	.price {
		font-size: 40rpx;
		font-weight: bold;
		color: #ff6600;
	}
	
	.original-price {
		font-size: 24rpx;
		color: #999999;
		text-decoration: line-through;
		margin-left: 10rpx;
	}
	
	.provider-section {
		background-color: #ffffff;
		margin-bottom: 20rpx;
		padding: 30rpx;
	}
	
	.provider-info {
		display: flex;
		align-items: center;
		margin-top: 20rpx;
	}
	
	.provider-avatar {
		width: 80rpx;
		height: 80rpx;
		border-radius: 50%;
		margin-right: 20rpx;
	}
	
	.provider-details {
		flex: 1;
	}
	
	.provider-name {
		font-size: 30rpx;
		font-weight: bold;
		color: #333333;
		display: block;
		margin-bottom: 10rpx;
	}
	
	.rating {
		display: flex;
	}
	
	.rating-score {
		color: #ff9900;
		font-weight: bold;
		margin-right: 20rpx;
	}
	
	.detail-section {
		background-color: #ffffff;
		margin-bottom: 20rpx;
		padding: 30rpx;
	}
	
	.section-title {
		font-size: 32rpx;
		font-weight: bold;
		color: #333333;
		margin-bottom: 30rpx;
	}
	
	.detail-content {
		display: flex;
		flex-direction: column;
		gap: 15rpx;
	}
	
	.detail-text {
		font-size: 28rpx;
		color: #666666;
		line-height: 1.6;
	}
	
	.bottom-bar {
		position: fixed;
		bottom: 0;
		left: 0;
		right: 0;
		height: 100rpx;
		background-color: #ffffff;
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 0 30rpx;
		box-shadow: 0 -2rpx 10rpx rgba(0,0,0,0.1);
	}
	
	.contact-button {
		width: 45%;
		height: 70rpx;
		background-color: #f0f0f0;
		color: #666666;
		border: none;
		border-radius: 35rpx;
		font-size: 28rpx;
	}
	
	.book-button {
		width: 45%;
		height: 70rpx;
		background-color: #00bfff;
		color: white;
		border: none;
		border-radius: 35rpx;
		font-size: 28rpx;
	}
</style>