<template>
	<view class="container">
		<!-- 订单状态筛选 -->
		<view class="status-filter">
			<scroll-view class="filter-scroll" scroll-x>
				<view class="filter-item" 
					  :class="{active: activeFilter === index}" 
					  v-for="(filter, index) in filters" 
					  :key="index" 
					  @click="switchFilter(index)">
					{{ filter }}
				</view>
			</scroll-view>
		</view>
		
		<!-- 订单列表 -->
		<view class="orders-list">
			<view class="order-item" v-for="(order, index) in filteredOrders" :key="order.order_id">
				<view class="order-header">
					<text class="order-number">订单号：{{ order.order_id }}</text>
					<text class="order-status" :class="'status-' + getOrderStatusType(order.order_status)">{{ getOrderStatusLabel(order.order_status) }}</text>
				</view>
				
				<navigator class="order-content" :url="'/pages/service-detail/service-detail?id=' + order.service_id">
					<image class="order-image" :src="order.image || '/static/service-default.jpg'"></image>
					<view class="order-info">
						<text class="order-service-name">{{ order.service_name || '家政服务' }}</text>
						<text class="order-time">预约时间：{{ order.service_date }} {{ getTimeSlotLabel(order.time_slot) }}</text>
						<text class="order-address">服务地址：{{ order.address || '北京市朝阳区xxx小区x号楼xxx室' }}</text>
					</view>
				</navigator>
				
				<view class="order-footer">
					<text class="order-price">实付金额：￥{{ order.total_amount }}</text>
					<view class="order-actions">
						<button v-if="order.order_status === 'pending'" class="action-button cancel-button" size="mini" @click="cancelOrder(order)">取消订单</button>
						<button v-if="order.order_status === 'pending'" class="action-button pay-button" size="mini" @click="payOrder(order)">立即支付</button>
						<button v-if="order.order_status === 'completed'" class="action-button comment-button" size="mini" @click="commentOrder(order)">评价</button>
					</view>
				</view>
			</view>
		</view>
		
		<!-- 空状态 -->
		<view class="empty-state" v-if="filteredOrders.length === 0">
			<image class="empty-icon" src="/static/empty-orders.png"></image>
			<text class="empty-text">暂无订单</text>
		</view>
		
		<!-- 下拉刷新和上拉加载 -->
		<view class="loading-more" v-if="loading">
			<text>加载中...</text>
		</view>
	</view>
</template>

<script>
	import { orderApi } from '../../api/api.js';
	
	export default {
		data() {
			return {
				activeFilter: 0,
				filters: ['全部', '待付款', '待服务', '服务中', '已完成'],
				allOrders: [],
				orders: [],
				page: 1,
				pageSize: 10,
				total: 0,
				loading: false,
				hasMore: true
			}
		},
		computed: {
			filteredOrders() {
				if (this.activeFilter === 0) {
					return this.orders;
				}
				
				const statusMap = {
					1: 'pending',     // 待付款
					2: 'confirmed',   // 待服务
					3: 'assigned',    // 服务中
					4: 'completed'    // 已完成
				};
				
				const statusKey = statusMap[this.activeFilter];
				return this.orders.filter(order => order.order_status === statusKey);
			}
		},
		onLoad() {
			this.loadOrders();
		},
		onReachBottom() {
			// 上拉加载更多
			if (this.hasMore && !this.loading) {
				this.loadMoreOrders();
			}
		},
		onPullDownRefresh() {
			// 下拉刷新
			this.refreshOrders();
		},
		methods: {
			async loadOrders() {
				if (this.loading) return;
				
				this.loading = true;
				try {
					const params = {
						page: this.page,
						page_size: this.pageSize
					};
					
					const result = await orderApi.getOrders(params);
					this.orders = result.orders || result;
					this.total = result.total || 0;
					this.hasMore = this.orders.length < this.total;
					
					uni.stopPullDownRefresh();
				} catch (err) {
					console.error('获取订单列表失败:', err);
					uni.showToast({
						title: '获取订单列表失败',
						icon: 'none'
					});
					
					uni.stopPullDownRefresh();
				} finally {
					this.loading = false;
				}
			},
			
			async refreshOrders() {
				this.page = 1;
				this.hasMore = true;
				await this.loadOrders();
			},
			
			async loadMoreOrders() {
				if (!this.hasMore) return;
				
				this.page++;
				this.loading = true;
				
				try {
					const params = {
						page: this.page,
						page_size: this.pageSize
					};
					
					const result = await orderApi.getOrders(params);
					const newOrders = result.orders || result;
					this.orders = [...this.orders, ...newOrders];
					this.total = result.total || 0;
					this.hasMore = this.orders.length < this.total;
				} catch (err) {
					console.error('加载更多订单失败:', err);
					uni.showToast({
						title: '加载更多订单失败',
						icon: 'none'
					});
					this.page--;
				} finally {
					this.loading = false;
				}
			},
			
			switchFilter(index) {
				this.activeFilter = index;
			},
			
			getOrderStatusLabel(status) {
				const statusMap = {
					'pending': '待付款',
					'confirmed': '待服务',
					'assigned': '已指派',
					'ongoing': '服务中',
					'completed': '已完成',
					'cancelled': '已取消'
				};
				return statusMap[status] || status;
			},
			
			getOrderStatusType(status) {
				const typeMap = {
					'pending': 1,
					'confirmed': 2,
					'assigned': 3,
					'ongoing': 3,
					'completed': 4,
					'cancelled': 5
				};
				return typeMap[status] || 0;
			},
			
			getTimeSlotLabel(timeSlot) {
				const timeSlotMap = {
					'morning': '上午(09:00-12:00)',
					'afternoon': '下午(12:00-18:00)',
					'evening': '晚上(18:00-22:00)',
					'full_day': '全天(09:00-22:00)'
				};
				return timeSlotMap[timeSlot] || timeSlot;
			},
			
			cancelOrder(order) {
				uni.showModal({
					title: '取消订单',
					content: '确定要取消该订单吗？',
					success: (res) => {
						if (res.confirm) {
							uni.showToast({
								title: '订单已取消',
								icon: 'success'
							});
							// 实际项目中这里应该调用接口取消订单
						}
					}
				});
			},
			
			payOrder(order) {
				uni.showToast({
					title: '正在跳转支付...',
					icon: 'none'
				});
				// 实际项目中这里应该跳转到支付页面
			},
			
			commentOrder(order) {
				uni.showToast({
					title: '评价功能开发中',
					icon: 'none'
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
	
	.status-filter {
		background-color: #ffffff;
		padding: 20rpx 0;
		margin-bottom: 20rpx;
	}
	
	.filter-scroll {
		white-space: nowrap;
	}
	
	.filter-item {
		display: inline-block;
		padding: 15rpx 30rpx;
		margin: 0 15rpx;
		font-size: 28rpx;
		color: #666666;
		border-radius: 30rpx;
	}
	
	.filter-item.active {
		background-color: #00bfff;
		color: #ffffff;
	}
	
	.orders-list {
		padding: 0 20rpx;
	}
	
	.order-item {
		background-color: #ffffff;
		border-radius: 10rpx;
		margin-bottom: 20rpx;
		box-shadow: 0 2rpx 10rpx rgba(0,0,0,0.1);
		overflow: hidden;
	}
	
	.order-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 20rpx;
		border-bottom: 1rpx solid #eeeeee;
	}
	
	.order-number {
		font-size: 28rpx;
		color: #333333;
	}
	
	.order-status {
		font-size: 24rpx;
		padding: 5rpx 15rpx;
		border-radius: 20rpx;
	}
	
	.status-1 {
		background-color: #ff9900;
		color: #ffffff;
	}
	
	.status-2 {
		background-color: #1aad19;
		color: #ffffff;
	}
	
	.status-3 {
		background-color: #00bfff;
		color: #ffffff;
	}
	
	.status-4 {
		background-color: #888888;
		color: #ffffff;
	}
	
	.status-5 {
		background-color: #cccccc;
		color: #666666;
	}
	
	.order-content {
		display: flex;
		padding: 20rpx;
		border-bottom: 1rpx solid #eeeeee;
	}
	
	.order-image {
		width: 120rpx;
		height: 120rpx;
		border-radius: 10rpx;
		margin-right: 20rpx;
	}
	
	.order-info {
		flex: 1;
		display: flex;
		flex-direction: column;
		justify-content: space-between;
	}
	
	.order-service-name {
		font-size: 30rpx;
		font-weight: bold;
		color: #333333;
		margin-bottom: 10rpx;
	}
	
	.order-time, .order-address {
		font-size: 24rpx;
		color: #666666;
		margin-bottom: 5rpx;
	}
	
	.order-footer {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 20rpx;
	}
	
	.order-price {
		font-size: 32rpx;
		font-weight: bold;
		color: #ff6b6b;
	}
	
	.order-actions {
		display: flex;
		gap: 10rpx;
	}
	
	.action-button {
		font-size: 24rpx;
		padding: 10rpx 20rpx;
		border-radius: 10rpx;
	}
	
	.cancel-button {
		background-color: #ffffff;
		color: #ff6b6b;
		border: 1rpx solid #ff6b6b;
	}
	
	.pay-button {
		background-color: #00bfff;
		color: #ffffff;
		border: none;
	}
	
	.comment-button {
		background-color: #ff9900;
		color: #ffffff;
		border: none;
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
	}
	
	.empty-text {
		font-size: 28rpx;
		color: #999999;
	}
	
	.loading-more {
		text-align: center;
		padding: 20rpx;
		font-size: 24rpx;
		color: #999999;
	}
</style>