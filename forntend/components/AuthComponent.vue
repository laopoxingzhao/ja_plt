<template>
	<view class="auth-container">
		<view class="auth-tabs">
			<view class="tab" :class="{ active: activeTab === 'login' }" @click="switchTab('login')">
				<text class="tab-text">登录</text>
			</view>
			<view class="tab" :class="{ active: activeTab === 'register' }" @click="switchTab('register')">
				<text class="tab-text">注册</text>
			</view>
		</view>

		<view class="auth-form">
			<!-- 登录表单 -->
			<view v-if="activeTab === 'login'">
				<input class="input-field" type="text" placeholder="用户名或邮箱" v-model="loginForm.identifier" />
				<input class="input-field" type="password" placeholder="密码" v-model="loginForm.password" />
				<button class="auth-button" @click="handleLogin">
					<text class="button-text">登录</text>
				</button>
			</view>

			<!-- 注册表单 -->
			<view v-else>
				<input class="input-field" type="text" placeholder="用户名" v-model="registerForm.username" />
				<input class="input-field" type="text" placeholder="邮箱" v-model="registerForm.email" />
				<input class="input-field" type="text" placeholder="手机号" v-model="registerForm.phone" />
				<input class="input-field" type="password" placeholder="密码" v-model="registerForm.password" />
				<button class="auth-button" @click="handleRegister">
					<text class="button-text">注册</text>
				</button>
			</view>
		</view>
	</view>
</template>

<script>
	import {
		userApi
	} from '../api/api.js';

	export default {
		name: 'AuthComponent',
		data() {
			return {
				activeTab: 'login',
				loginFocus: false,
				registerFocus: false,
				loginForm: {
					identifier: '',
					password: ''
				},
				registerForm: {
					username: '',
					email: '',
					phone: '',
					password: ''
				}
			}
		},
		methods: {
			switchTab(tab) {
				this.activeTab = tab
				// set focus flags (used by native platforms that respect :focus)
				this.loginFocus = tab === 'login';
				this.registerFocus = tab === 'register';
				this.$nextTick(() => {
					try {
						if (tab === 'login' && this.$refs.loginIdentifier) {
							this.$refs.loginIdentifier.focus && this.$refs.loginIdentifier.focus();
						} else if (tab === 'register' && this.$refs.registerUsername) {
							this.$refs.registerUsername.focus && this.$refs.registerUsername.focus();
						}
					} catch (e) {
						// 某些平台/环境下 refs.focus 可能不可用，捕获以免崩溃
						console.warn('focus failed', e);
					}
				});
			},

			onFocus(refName) {
				// 占位，可用于收起遮罩/改变样式等
				// console.log('focus', refName)
			},

			onBlur(refName) {
				// 占位
				// console.log('blur', refName)
			},

			async handleLogin() {
				if (!this.loginForm.identifier || !this.loginForm.password) {
					uni.showToast({
						title: '请填写完整信息',
						icon: 'none'
					});
					return;
				}

				try {
					const res = await userApi.login(this.loginForm);
					uni.setStorageSync('token', res.token);
					uni.setStorageSync('user', JSON.stringify(res.user));

					uni.showToast({
						title: '登录成功',
						icon: 'success'
					});

					// 触发登录成功事件
					this.$emit('login-success', res.user);
				} catch (err) {
					uni.showToast({
						title: err.message || '登录失败',
						icon: 'none'
					});
				}
			},

			async handleRegister() {
				if (!this.registerForm.username ||
					!this.registerForm.email ||
					!this.registerForm.phone ||
					!this.registerForm.password) {
					uni.showToast({
						title: '请填写完整信息',
						icon: 'none'
					});
					return;
				}

				// 简单验证手机号
				if (!/^1[3-9]\d{9}$/.test(this.registerForm.phone)) {
					uni.showToast({
						title: '请输入正确的手机号',
						icon: 'none'
					});
					return;
				}

				// 简单验证邮箱
				if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(this.registerForm.email)) {
					uni.showToast({
						title: '请输入正确的邮箱地址',
						icon: 'none'
					});
					return;
				}

				try {
					const res = await userApi.register(this.registerForm);
					uni.showToast({
						title: '注册成功',
						icon: 'success'
					});

					// 切换到登录标签
					this.activeTab = 'login';

					// 清空注册表单
					this.registerForm = {
						username: '',
						email: '',
						phone: '',
						password: ''
					};

					// 触发注册成功事件
					this.$emit('register-success', res.user.username);
				} catch (err) {
					uni.showToast({
						title: err.message || '注册失败',
						icon: 'none'
					});
				}
			}
		}
	}
</script>

<style scoped>
	.auth-container {
		padding: 40rpx;
		background-color: #ffffff;
		margin: 20rpx;
		border: 2rpx solid #eeeeee;
		border-radius: 10rpx;
	}

	.auth-tabs {
		display: flex;
		border-bottom: 2rpx solid #eeeeee;
		margin-bottom: 40rpx;
	}

	.tab {
		flex: 1;
		text-align: center;
		padding: 30rpx;
	}

	.tab-text {
		font-size: 32rpx;
		color: #999999;
		font-weight: 500;
	}

	.tab.active .tab-text {
		color: #00bfff;
		font-weight: 600;
	}

	.input-field {
		
		padding: 25rpx;
		margin-bottom: 25rpx;
		border: 2rpx solid #dddddd;
		border-radius: 8rpx;
		
		font-size: 28rpx;
		background-color: #ffffff;
		color: #333333;
	}

	.input-field::placeholder {
		color: #999999;
	}

	.input-field:focus {
		border-color: #00bfff;
		outline: none;
	}

	.auth-button {
		width: 100%;
		padding: 25rpx;
		background-color: #00bfff;
		border: none;
		border-radius: 8rpx;
		margin-top: 20rpx;
	}

	.button-text {
		font-size: 32rpx;
		color: #ffffff;
		font-weight: 500;
	}
</style>