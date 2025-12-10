<template>
	<view class="container">
		<!-- 头部搜索区域 -->
		<view class="header">
			<view class="search-box">
				<input class="search-input" placeholder="搜索家政服务..." v-model="searchKeyword" @confirm="searchServices"/>
			</view>
		</view>
		
		<!-- 轮播图 -->
		<view class="swiper-section">
			<swiper class="swiper" indicator-dots autoplay interval="5000" duration="1000" circular>
				<swiper-item v-for="(item, index) in bannerImages" :key="index">
					<view class="swiper-item">
						<view class="virtual-banner" :class="'banner-' + (index + 1)">
							<view class="banner-content">
								<text class="banner-title">{{ item.title }}</text>
								<text class="banner-desc">{{ item.desc }}</text>
							</view>
						</view>
					</view>
				</swiper-item>
			</swiper>
		</view>
		
		<!-- 服务分类 -->
		<view class="service-category">
			<view class="category-title">服务分类</view>
			<view class="category-grid">
				<view class="category-item" v-for="(item, index) in serviceCategories" :key="index" @click="goToService(item)">
					<image class="category-icon" :src="item.icon"></image>
					<text class="category-text">{{ item.name }}</text>
				</view>
			</view>
		</view>
		
		<!-- 推荐服务 -->
		<view class="recommended-services">
			<view class="section-title">推荐服务</view>
			<view class="service-list">
				<view class="service-card" v-for="(service, index) in recommendedServices" :key="service.service_id" @click="goToServiceDetail(service)">
					<image class="service-image" :src="service.image || '/static/service-default.jpg'"></image>
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
		</view>
	</view>
</template>

<script>
	import { serviceApi } from '../../api/api.js';
	
	export default {
		data() {
			return {
				title: '家政服务',
				searchKeyword: '',
				bannerImages: [
					{
						title: '专业保洁服务',
						desc: '高品质家庭清洁，让您的家焕然一新'
					},
					{
						title: '贴心月嫂服务',
						desc: '专业培训月嫂，为您提供贴心服务'
					},
					{
						title: '快速维修服务',
						desc: '24小时上门维修，解决您的燃眉之急'
					}
				],
				serviceCategories: [],
				recommendedServices: []
			}
		},
		
		onLoad() {
			this.loadServiceCategories();
			this.loadRecommendedServices();
		},
		
		methods: {
			async loadServiceCategories() {
				try {
					const categories = await serviceApi.getCategories();
					this.serviceCategories = categories;
				} catch (err) {
					console.error('获取服务分类失败:', err);
					uni.showToast({
						title: '获取服务分类失败',
						icon: 'none'
					});
					
				}
			},
			
			async loadRecommendedServices() {
				console.log('加载推荐服务...');
				
				try {
					const result = await serviceApi.getServices({ page_size: 3 });
					// 只显示前3个服务作为推荐
					this.recommendedServices = result.services || result.slice(0, 3);
				} catch (err) {
					console.error('获取服务列表失败:', err);
					uni.showToast({
						title: '获取服务列表失败',
						icon: 'none'
					});
					
				}
			},
			
			goToService(category) {
				uni.showToast({
					title: `跳转到${category.name}服务`,
					icon: 'none'
				});
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
			},
			
			searchServices() {
				if (this.searchKeyword.trim()) {
					uni.showToast({
						title: `搜索"${this.searchKeyword}"`,
						icon: 'none'
					});
				}
			}
		}
	}
</script>

<style>
	.container {
		background-color: #f5f5f5;
		min-height: 100vh;
		padding-bottom: 20rpx;
	}
	
	.header {
		background-color: #ffffff;
		padding: 20rpx;
		position: sticky;
		top: 0;
		z-index: 999;
		box-shadow: 0 2rpx 10rpx rgba(0,0,0,0.1);
	}
	
	.search-box {
		background-color: #f0f0f0;
		border-radius: 40rpx;
		padding: 15rpx 30rpx;
	}
	
	.search-input {
		font-size: 28rpx;
		color: #666666;
	}
	
	.swiper-section {
		padding: 20rpx;
		background-color: #ffffff;
	}
	
	.swiper {
		height: 300rpx;
		border-radius: 20rpx;
	}

	.swiper-item {
		display: flex;
		justify-content: center;
		align-items: center;
		height: 100%;
	}

	.virtual-banner {
		width: 100%;
		height: 100%;
		border-radius: 20rpx;
		display: flex;
		align-items: center;
		justify-content: center;
		position: relative;
	}

	.banner-content {
		text-align: center;
		color: white;
		z-index: 2;
	}

	.banner-title {
		font-size: 36rpx;
		font-weight: bold;
		display: block;
		margin-bottom: 10rpx;
		text-shadow: 1rpx 1rpx 2rpx rgba(0,0,0,0.5);
	}

	.banner-desc {
		font-size: 28rpx;
		display: block;
		text-shadow: 1rpx 1rpx 2rpx rgba(0,0,0,0.5);
	}

	.banner-1 {
		background: linear-gradient(45deg, #00bfff, #1e90ff);
	}

	.banner-2 {
		background: linear-gradient(45deg, #ff6b6b, #ff8e53);
	}

	.banner-3 {
		background: linear-gradient(45deg, #4facfe, #00f2fe);
	}

	.service-category {
		background-color: #ffffff;
		margin: 20rpx;
		border-radius: 10rpx;
		padding: 20rpx;
		box-shadow: 0 2rpx 10rpx rgba(0,0,0,0.1);
	}

	.category-title {
		font-size: 32rpx;
		font-weight: bold;
		color: #333333;
		margin-bottom: 20rpx;
	}

	.category-grid {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: 20rpx;
	}

	.category-item {
		display: flex;
		flex-direction: column;
		align-items: center;
	}

	.category-icon {
		width: 80rpx;
		height: 80rpx;
		margin-bottom: 10rpx;
	}

	.category-text {
		font-size: 24rpx;
		color: #666666;
		text-align: center;
	}

	.recommended-services {
		background-color: #ffffff;
		margin: 20rpx;
		border-radius: 10rpx;
		padding: 20rpx;
		box-shadow: 0 2rpx 10rpx rgba(0,0,0,0.1);
	}

	.section-title {
		font-size: 32rpx;
		font-weight: bold;
		color: #333333;
		margin-bottom: 20rpx;
	}

	.service-list {
		display: flex;
		flex-direction: column;
		gap: 20rpx;
	}

	.service-card {
		display: flex;
		border: 1rpx solid #eeeeee;
		border-radius: 10rpx;
		overflow: hidden;
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
		font-size: 30rpx;
		font-weight: bold;
		color: #333333;
		margin-bottom: 10rpx;
	}

	.service-description {
		font-size: 24rpx;
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
		font-size: 32rpx;
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
</style>