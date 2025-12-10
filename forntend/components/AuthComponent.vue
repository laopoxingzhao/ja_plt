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
        <button class="logout-button" @click="handleLogout">登出</button>
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
  </view>
</template>

<script>
import { userApi } from '../api/api.js';

export default {
  name: 'AuthComponent',
  data() {
    return {
      activeTab: 'login',
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
      this.activeTab = tab;
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
  background: white;
  border-radius: 10px;
  box-shadow: 0 15px 35px rgba(0, 0, 0, 0.1);
  overflow: hidden;
  width: 100%;
  max-width: 400px;
}

.auth-tabs {
  display: flex;
  border-bottom: 1px solid #eee;
}

.tab {
  flex: 1;
  text-align: center;
  padding: 20px;
  cursor: pointer;
  font-weight: 600;
  color: #666;
  transition: all 0.3s ease;
}

.tab.active {
  color: #00bfff;
  border-bottom: 3px solid #00bfff;
}

.auth-form {
  padding: 30px;
}

.input-field {
  width: 100%;
  padding: 15px;
  margin-bottom: 15px;
  border: 1px solid #ddd;
  border-radius: 5px;
  box-sizing: border-box;
}

.auth-button {
  width: 100%;
  padding: 15px;
  background-color: #00bfff;
  color: white;
  border: none;
  border-radius: 5px;
  font-size: 16px;
  cursor: pointer;
}
</style>