<template>
	<view class="container">
		<!-- 页面标题 -->
		<view class="page-header">
			<text class="page-title">{{ categoryName || '服务列表' }}</text>
		</view>
		
		<!-- 服务列表 -->
		<view class="service-list" v-if="services.length > 0">
			<view class="service-card" v-for="service in services" :key="service.service_id" @click="goToServiceDetail(service)">
				<image class="service-image" :src="service.image || '/static/service-default.jpg'" mode="aspectFill"></image>
				<view class="service-info">
					<text class="service-name">{{ service.service_name }}</text>
					<text class="service-description">{{ service.description || '专业服务' }}</text>
					<view class="service-footer">
						<text class="service-price">￥{{ service.base_price }}</text>
						<button class="book-button" size="mini" @click.stop="bookService(service)">立即预约</button>
					</view>
				</view>
			</view>
		</view>
		
		<!-- 空状态 -->
		<view class="empty-state" v-else>
			<image class="empty-icon" src="/static/icons/empty.svg" mode="aspectFit"></image>
			<text class="empty-text">暂无相关服务</text>
		</view>
		
		<!-- 加载更多 -->
		<view class="load-more" v-if="hasMore && services.length > 0">
			<text class="load-more-text" @click="loadMore">点击加载更多</text>
		</view>
	</view>
</template>

<script>
	import { serviceApi } from '../../api/api.js';
	
	export default {
		data() {
			return {
				categoryId: null,
				categoryName: '',
				services: [],
				currentPage: 1,
				pageSize: 10,
				total: 0,
				hasMore: false
			}
		},
		
		onLoad(options) {
			if (options.categoryId) {
				this.categoryId = parseInt(options.categoryId);
			}
			
			if (options.categoryName) {
				this.categoryName = decodeURIComponent(options.categoryName);
			}
			
			this.loadServices();
		},
		
		onReachBottom() {
			// 触底加载更多
			if (this.hasMore) {
				this.loadMore();
			}
		},
		
		methods: {
			async loadServices() {
				try {
					const params = {
						page: this.currentPage,
						page_size: this.pageSize
					};
					
					// 如果指定了分类ID，则添加到参数中
					if (this.categoryId && this.categoryId !== 8) { // 8是"全部服务"
						params.category_id = this.categoryId;
					}
					
					const result = await serviceApi.getServices(params);
					
					if (this.currentPage === 1) {
						this.services = result.services || [];
					} else {
						this.services = [...this.services, ...(result.services || [])];
					}
					
					this.total = result.total || 0;
					this.hasMore = this.services.length < this.total;
				} catch (err) {
					console.error('获取服务列表失败:', err);
					uni.showToast({
						title: '获取服务列表失败',
						icon: 'none'
					});
				}
			},
			
			loadMore() {
				if (!this.hasMore) return;
				
				this.currentPage++;
				this.loadServices();
			},
			
			goToServiceDetail(service) {
				uni.navigateTo({
					url: `/pages/service-detail/service-detail?id=${service.service_id}`
				});
			},
			
			bookService(service) {
				uni.navigateTo({
					url: `/pages/booking/booking?id=${service.service_id}`
				});
			}
		}
	}
</script>

<style>
	.container {
		background-color: #f5f5f5;
		min-height: 100vh;
		padding: 20rpx;
	}
	
	.page-header {
		background-color: #ffffff;
		padding: 30rpx 20rpx;
		margin-bottom: 20rpx;
		border-radius: 10rpx;
		box-shadow: 0 2rpx 10rpx rgba(0, 0, 0, 0.1);
		text-align: center;
	}
	
	.page-title {
		font-size: 36rpx;
		font-weight: bold;
		color: #333333;
	}
	
	.service-list {
		display: flex;
		flex-direction: column;
		gap: 20rpx;
	}
	
	.service-card {
		background-color: #ffffff;
		border-radius: 10rpx;
		overflow: hidden;
		box-shadow: 0 2rpx 10rpx rgba(0, 0, 0, 0.1);
		display: flex;
	}
	
	.service-image {
		width: 200rpx;
		height: 200rpx;
	}
	
	.service-info {
		flex: 1;
		padding: 20rpx;
		display: flex;
		flex-direction: column;
		justify-content: space-between;
	}
	
	.service-name {
		font-size: 32rpx;
		font-weight: bold;
		color: #333333;
		margin-bottom: 10rpx;
	}
	
	.service-description {
		font-size: 26rpx;
		color: #666666;
		margin-bottom: 20rpx;
		flex: 1;
	}
	
	.service-footer {
		display: flex;
		justify-content: space-between;
		align-items: center;
	}
	
	.service-price {
		font-size: 36rpx;
		font-weight: bold;
		color: #ff6b6b;
	}
	
	.book-button {
		background-color: #00bfff;
		color: white;
		border: none;
		border-radius: 10rpx;
		padding: 10rpx 20rpx;
		font-size: 24rpx;
	}
	
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 100rpx 0;
	}
	
	.empty-icon {
		width: 200rpx;
		height: 200rpx;
		margin-bottom: 30rpx;
		opacity: 0.6;
	}
	
	.empty-text {
		font-size: 30rpx;
		color: #999999;
	}
	
	.load-more {
		text-align: center;
		padding: 30rpx 0;
	}
	
	.load-more-text {
		font-size: 28rpx;
		color: #00bfff;
	}
</style>