<template>
  <view class="auth-container">
    <view class="auth-tabs">
      <view 
        class="tab" 
        :class="{ active: activeTab === 'login' }"
        @click="switchTab('login')"
      >
        登录
      </view>
      <view 
        class="tab" 
        :class="{ active: activeTab === 'register' }"
        @click="switchTab('register')"
      >
        注册
      </view>
    </view>

    <view class="auth-form">
      <!-- 登录表单 -->
      <view v-if="activeTab === 'login'">
        <input 
          class="input-field" 
          placeholder="用户名或邮箱" 
          v-model="loginForm.identifier"
        />
        <input 
          class="input-field" 
          placeholder="密码" 
          password 
          v-model="loginForm.password"
        />
        <button class="auth-button" @click="handleLogin">登录</button>
      </view>

      <!-- 注册表单 -->
      <view v-else>
        <input 
          class="input-field" 
          placeholder="用户名" 
          v-model="registerForm.username"
        />
        <input 
          class="input-field" 
          placeholder="邮箱" 
          v-model="registerForm.email"
        />
        <input 
          class="input-field" 
          placeholder="手机号" 
          v-model="registerForm.phone"
        />
        <input 
          class="input-field" 
          placeholder="密码" 
          password 
          v-model="registerForm.password"
        />
        <button class="auth-button" @click="handleRegister">注册</button>
      </view>
    </view>
    
    <!-- 登出按钮应该在用户已登录的情况下显示 -->
    <view v-if="isLoggedIn" class="logout-section">
      <button class="logout-button" @click="handleLogout">登出</button>
    </view>
  </view>
</template>

<script>
import { userApi } from '../api/api.js';

export default {
  name: 'AuthComponent',
  data() {
    return {
      activeTab: 'login',
      isLoggedIn: false,
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
  mounted() {
    // 检查用户是否已登录
    const token = uni.getStorageSync('token')
    this.isLoggedIn = !!token
  },
  methods: {
    switchTab(tab) {
      this.activeTab = tab
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
        this.isLoggedIn = true
        
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
    },
    
    async handleLogout() {
      try {
        // 调用登出API
        await userApi.logout();
        
        // 清除本地存储的token和用户信息
        uni.removeStorageSync('token');
        uni.removeStorageSync('user');
        this.isLoggedIn = false
        
        uni.showToast({
          title: '登出成功',
          icon: 'success'
        });
        
        // 触发登出成功事件
        this.$emit('logout-success');
      } catch (err) {
        uni.showToast({
          title: err.message || '登出失败',
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
  background-color: #fff;
}

.auth-tabs {
  display: flex;
  border-bottom: 2rpx solid #eee;
  margin-bottom: 40rpx;
}

.tab {
  flex: 1;
  text-align: center;
  padding: 20rpx;
  font-size: 32rpx;
  color: #666;
}

.tab.active {
  color: #00bfff;
  border-bottom: 4rpx solid #00bfff;
}

.input-field {
  width: 100%;
  height: 80rpx;
  border: 2rpx solid #eee;
  border-radius: 10rpx;
  padding: 0 20rpx;
  margin-bottom: 20rpx;
  font-size: 28rpx;
}

.auth-button {
  width: 100%;
  height: 80rpx;
  background-color: #00bfff;
  color: #fff;
  border-radius: 10rpx;
  font-size: 32rpx;
  margin-top: 20rpx;
}

.logout-button {
  width: 100%;
  height: 80rpx;
  background-color: #ff4d4f;
  color: #fff;
  border-radius: 10rpx;
  font-size: 32rpx;
  margin-top: 20rpx;
}
</style>