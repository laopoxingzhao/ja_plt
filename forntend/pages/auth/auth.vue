<template>
	<view class="container">
		<view class="header">
			<text class="title">用户登录</text>
		</view>
		
		<AuthComponent @login-success="onLoginSuccess" @register-success="onRegisterSuccess" />
		
		<view class="footer">
			<navigator url="/pages/index/index" class="link">返回首页</navigator>
		</view>
	</view>
</template>

<script>
	import AuthComponent from '@/components/AuthComponent.vue';
	
	export default {
		components: {
			AuthComponent
		},
		data() {
			return {
			}
		},
		methods: {
			onLoginSuccess(user) {
				uni.showToast({
					title: '登录成功',
					icon: 'success'
				});
				
				// 返回上一页或跳转到用户中心
				setTimeout(() => {
					uni.navigateBack();
				}, 1000);
			},
			
			onRegisterSuccess(username) {
				uni.showToast({
					title: '注册成功',
					icon: 'success'
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
	
	.header {
		text-align: center;
		padding: 40rpx 0;
	}
	
	.title {
		font-size: 48rpx;
		font-weight: bold;
		color: #333;
	}
	
	.footer {
		text-align: center;
		padding: 40rpx 0;
	}
	
	.link {
		color: #00bfff;
		font-size: 28rpx;
	}
</style>