<template>
	<view class="container">
		<!-- 服务信息 -->
		<view class="service-summary">
			<image class="service-image" :src="service.image || '/static/service-default.jpg'"></image>
			<view class="service-info">
				<text class="service-name">{{ service.service_name }}</text>
				<text class="service-price">￥{{ service.base_price }}</text>
			</view>
		</view>
		
		<!-- 预约时间选择 -->
		<view class="booking-section">
			<view class="section-title">预约时间</view>
			<view class="date-selector">
				<scroll-view class="date-scroll" scroll-x>
					<view class="date-item" 
						  :class="{active: selectedIndex === index}" 
						  v-for="(date, index) in availableDates" 
						  :key="index" 
						  @click="selectDate(index)">
						<text class="week">{{ date.week }}</text>
						<text class="day">{{ date.day }}</text>
						<text class="month">{{ date.month }}</text>
					</view>
				</scroll-view>
			</view>
			
			<view class="time-slots">
				<view class="time-slot" 
					  :class="{active: selectedTimeIndex === index}"
					  v-for="(time, index) in timeSlots" 
					  :key="index" 
					  @click="selectTime(index)">
					{{ time }}
				</view>
			</view>
		</view>
		
		<!-- 服务地址 -->
		<view class="address-section">
			<view class="section-title">服务地址</view>
			<view class="address-form">
				<input class="input-field" placeholder="请输入联系人姓名" v-model="contactName" />
				<input class="input-field" placeholder="请输入联系电话" v-model="contactPhone" />
				<input class="input-field" placeholder="请输入省市区" v-model="region" />
				<textarea class="textarea-field" placeholder="请输入详细地址" v-model="addressDetail" />
			</view>
		</view>
		
		<!-- 备注信息 -->
		<view class="remark-section">
			<view class="section-title">备注信息</view>
			<textarea class="remark-textarea" placeholder="如有特殊要求，请在此说明..." v-model="remark" />
		</view>
		
		<!-- 底部结算栏 -->
		<view class="bottom-bar">
			<view class="total-amount">
				合计：<text class="amount">￥{{ service.base_price }}</text>
			</view>
			<button class="submit-button" @click="submitOrder">提交订单</button>
		</view>
	</view>
</template>

<script>
	import { serviceApi, orderApi } from '../../api/api.js';
	
	export default {
		data() {
			return {
				serviceId: null,
				service: {},
				selectedIndex: 0,
				selectedTimeIndex: 0,
				contactName: '',
				contactPhone: '',
				region: '',
				addressDetail: '',
				remark: '',
				availableDates: [],
				timeSlots: [
					'09:00-11:00',
					'11:00-13:00',
					'13:00-15:00',
					'15:00-17:00',
					'17:00-19:00',
					'19:00-21:00'
				]
			}
		},
		onLoad(options) {
			if (options.id) {
				this.serviceId = options.id;
				this.loadServiceDetail();
			}
			this.generateAvailableDates();
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
			
			generateAvailableDates() {
				const dates = [];
				const today = new Date();
				
				for (let i = 0; i < 7; i++) {
					const date = new Date(today);
					date.setDate(today.getDate() + i);
					
					const weekDays = ['周日', '周一', '周二', '周三', '周四', '周五', '周六'];
					const months = ['1月', '2月', '3月', '4月', '5月', '6月', '7月', '8月', '9月', '10月', '11月', '12月'];
					
					dates.push({
						week: i === 0 ? '今天' : weekDays[date.getDay()],
						day: date.getDate(),
						month: months[date.getMonth()]
					});
				}
				
				this.availableDates = dates;
			},
			
			selectDate(index) {
				this.selectedIndex = index;
			},
			
			selectTime(index) {
				this.selectedTimeIndex = index;
			},
			
			async submitOrder() {
				// 表单验证
				if (!this.contactName || !this.contactPhone || !this.region || !this.addressDetail) {
					uni.showToast({
						title: '请填写完整地址信息',
						icon: 'none'
					});
					return;
				}
				
				if (!/^1[3-9]\d{9}$/.test(this.contactPhone)) {
					uni.showToast({
						title: '请输入正确的手机号',
						icon: 'none'
					});
					return;
				}
				
				// 构造订单数据
				const orderData = {
					customer_id: this.getUserId(),
					service_id: parseInt(this.serviceId),
					address_id: 1, // 实际项目中应该选择或创建地址
					service_date: this.formatSelectedDate(),
					time_slot: this.getTimeSlotValue(),
					duration: 1, // 默认时长
					unit_price: this.service.base_price,
					subtotal: this.service.base_price,
					discount_amount: 0,
					total_amount: this.service.base_price,
					payment_status: 'pending',
					order_status: 'pending',
					special_instructions: this.remark
				};
				
				try {
					const result = await orderApi.createOrder(orderData);
					uni.showToast({
						title: '订单提交成功',
						icon: 'success'
					});
					
					// 跳转到订单详情页
					setTimeout(() => {
						uni.navigateTo({
							url: `/pages/orders/orders`
						});
					}, 1500);
				} catch (err) {
					console.error('提交订单失败:', err);
					uni.showToast({
						title: '提交订单失败: ' + (err.message || ''),
						icon: 'none'
					});
				}
			},
			
			getUserId() {
				const userStr = uni.getStorageSync('user');
				if (userStr) {
					try {
						const user = JSON.parse(userStr);
						return user.user_id;
					} catch (e) {
						console.error('解析用户信息失败:', e);
					}
				}
				return null;
			},
			
			formatSelectedDate() {
				const today = new Date();
				const selectedDate = new Date(today);
				selectedDate.setDate(today.getDate() + this.selectedIndex);
				return selectedDate.toISOString().split('T')[0];
			},
			
			getTimeSlotValue() {
				const timeSlotMap = {
					0: 'morning',
					1: 'morning',
					2: 'afternoon',
					3: 'afternoon',
					4: 'evening',
					5: 'evening'
				};
				return timeSlotMap[this.selectedTimeIndex] || 'morning';
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
	
	.service-summary {
		background-color: #ffffff;
		display: flex;
		padding: 30rpx;
		margin-bottom: 20rpx;
	}
	
	.service-image {
		width: 150rpx;
		height: 150rpx;
		border-radius: 10rpx;
		margin-right: 20rpx;
	}
	
	.service-info {
		flex: 1;
		display: flex;
		flex-direction: column;
		justify-content: center;
	}
	
	.service-name {
		font-size: 32rpx;
		font-weight: bold;
		color: #333333;
		margin-bottom: 20rpx;
	}
	
	.service-price {
		font-size: 36rpx;
		font-weight: bold;
		color: #ff6600;
	}
	
	.booking-section {
		background-color: #ffffff;
		padding: 30rpx;
		margin-bottom: 20rpx;
	}
	
	.date-selector {
		margin: 30rpx 0;
	}
	
	.date-scroll {
		white-space: nowrap;
	}
	
	.date-item {
		display: inline-flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		width: 100rpx;
		height: 120rpx;
		background-color: #f5f5f5;
		border-radius: 10rpx;
		margin-right: 20rpx;
	}
	
	.date-item.active {
		background-color: #00bfff;
		color: #ffffff;
	}
	
	.week {
		display: block;
		font-size: 24rpx;
		color: #666666;
		margin-bottom: 10rpx;
	}
	
	.day {
		display: block;
		font-size: 36rpx;
		font-weight: bold;
		color: #333333;
		margin-bottom: 5rpx;
	}
	
	.month {
		display: block;
		font-size: 20rpx;
		color: #999999;
	}
	
	.time-slots {
		display: flex;
		flex-wrap: wrap;
	}
	
	.time-slots {
		display: flex;
		flex-wrap: wrap;
		justify-content: space-between;
	}
	
	.time-slot {
		width: 30%;
		text-align: center;
		padding: 20rpx 0;
		background-color: #f5f5f5;
		border-radius: 10rpx;
		margin-bottom: 20rpx;
		font-size: 28rpx;
	}
	
	.time-slot.active {
		background-color: #00bfff;
		color: #ffffff;
	}
	
	.address-section, .remark-section {
		background-color: #ffffff;
		padding: 30rpx;
		margin-bottom: 20rpx;
	}
	
	.address-form {
		margin-top: 20rpx;
	}
	
	.address-form, .remark-textarea {
		background-color: #f5f5f5;
		border-radius: 10rpx;
		padding: 20rpx;
	}
	
	.input-field, .textarea-field, .remark-textarea {
		width: 100%;
		padding: 20rpx;
		margin-bottom: 20rpx;
		background-color: #ffffff;
		border: 1rpx solid #ddd;
		border-radius: 10rpx;
		box-sizing: border-box;
		font-size: 28rpx;
	}
	
	.textarea-field, .remark-textarea {
		height: 150rpx;
		resize: none;
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
	
	.total-amount {
		font-size: 30rpx;
		color: #333333;
	}
	
	.amount {
		font-size: 36rpx;
		font-weight: bold;
		color: #ff6600;
	}
	
	.submit-button {
		width: 40%;
		height: 70rpx;
		background-color: #00bfff;
		color: white;
		border: none;
		border-radius: 35rpx;
		font-size: 28rpx;
	}
	
	.section-title {
		font-size: 32rpx;
		font-weight: bold;
		color: #333333;
	}
</style>