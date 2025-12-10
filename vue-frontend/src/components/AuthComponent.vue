<template>
  <div class="user-profile">
    <h2>用户信息</h2>
    <div id="user-details">
      <p><strong>用户ID:</strong> {{ user.user_id }}</p>
      <p><strong>用户名:</strong> {{ user.username }}</p>
      <p><strong>邮箱:</strong> {{ user.email }}</p>
      <p><strong>手机号:</strong> {{ user.phone }}</p>
      <p><strong>用户类型:</strong> {{ user.type }}</p>
      <p><strong>账户状态:</strong> {{ user.status }}</p>
    </div>
    <button class="logout-btn" @click="handleLogout">退出登录</button>
  </div>
</template>

<script>
export default {
  name: 'UserProfile',
  props: {
    user: {
      type: Object,
      required: true
    }
  },
  methods: {
    handleLogout() {
      this.$emit('logout')
    }
  }
}
</script>

<style scoped>
.user-profile {
  text-align: center;
}

.logout-btn {
  background: #dc3545;
  margin-top: 20px;
  width: 100%;
  padding: 12px;
  color: white;
  border: none;
  border-radius: 5px;
  font-size: 16px;
  font-weight: 600;
  cursor: pointer;
  transition: transform 0.2s ease;
}

.logout-btn:hover {
  transform: translateY(-2px);
}

.logout-btn:active {
  transform: translateY(0);
}
</style>
<template>
  <div class="register-form">
    <h2>用户注册</h2>
    <form @submit.prevent="handleRegister">
      <div class="input-group">
        <label for="register-username">用户名</label>
        <input 
          type="text" 
          id="register-username" 
          v-model="registerForm.username"
          required
        >
      </div>
      <div class="input-group">
        <label for="register-email">邮箱</label>
        <input 
          type="email" 
          id="register-email" 
          v-model="registerForm.email"
          required
        >
      </div>
      <div class="input-group">
        <label for="register-phone">手机号</label>
        <input 
          type="tel" 
          id="register-phone" 
          v-model="registerForm.phone"
          required
        >
      </div>
      <div class="input-group">
        <label for="register-password">密码</label>
        <input 
          type="password" 
          id="register-password" 
          v-model="registerForm.password"
          required
        >
      </div>
      <button type="submit">注册</button>
      <div 
        id="register-message" 
        class="message"
        :class="registerMessage.type"
        v-show="registerMessage.text"
      >
        {{ registerMessage.text }}
      </div>
    </form>
  </div>
</template>

<script>
export default {
  name: 'RegisterForm',
  data() {
    return {
      registerForm: {
        username: '',
        email: '',
        phone: '',
        password: ''
      },
      registerMessage: {
        type: '',
        text: ''
      }
    }
  },
  methods: {
    async handleRegister() {
      try {
        const response = await fetch('/api/users/register', {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json'
          },
          body: JSON.stringify(this.registerForm)
        })
        
        const data = await response.json()
        
        if (response.ok) {
          // 注册成功
          this.registerMessage.type = 'success'
          this.registerMessage.text = '注册成功！请登录'
          
          // 触发注册成功事件
          this.$emit('register-success', this.registerForm.username)
        } else {
          // 注册失败
          this.registerMessage.type = 'error'
          this.registerMessage.text = data || '注册失败'
        }
      } catch (error) {
        this.registerMessage.type = 'error'
        this.registerMessage.text = '网络错误，请稍后再试'
      }
    }
  }
}
</script>

<style scoped>
h2 {
  text-align: center;
  margin-bottom: 30px;
  color: #333;
}

.input-group {
  margin-bottom: 20px;
}

label {
  display: block;
  margin-bottom: 8px;
  color: #555;
  font-weight: 500;
}

input {
  width: 100%;
  padding: 12px 15px;
  border: 2px solid #e1e1e1;
  border-radius: 5px;
  font-size: 16px;
  transition: border-color 0.3s ease;
}

input:focus {
  outline: none;
  border-color: #667eea;
}

button {
  width: 100%;
  padding: 12px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border: none;
  border-radius: 5px;
  font-size: 16px;
  font-weight: 600;
  cursor: pointer;
  transition: transform 0.2s ease;
}

button:hover {
  transform: translateY(-2px);
}

button:active {
  transform: translateY(0);
}

.message {
  margin-top: 15px;
  padding: 10px;
  border-radius: 5px;
  text-align: center;
  display: none;
}

.message.success {
  background-color: #d4edda;
  color: #155724;
  border: 1px solid #c3e6cb;
  display: block;
}

.message.error {
  background-color: #f8d7da;
  color: #721c24;
  border: 1px solid #f5c6cb;
  display: block;
}
</style>
<template>
  <div class="login-form">
    <h2>用户登录</h2>
    <form @submit.prevent="handleLogin">
      <div class="input-group">
        <label for="login-identifier">用户名或邮箱</label>
        <input 
          type="text" 
          id="login-identifier" 
          v-model="loginForm.identifier"
          required
        >
      </div>
      <div class="input-group">
        <label for="login-password">密码</label>
        <input 
          type="password" 
          id="login-password" 
          v-model="loginForm.password"
          required
        >
      </div>
      <button type="submit">登录</button>
      <div 
        id="login-message" 
        class="message"
        :class="loginMessage.type"
        v-show="loginMessage.text"
      >
        {{ loginMessage.text }}
      </div>
    </form>
  </div>
</template>

<script>
export default {
  name: 'LoginForm',
  data() {
    return {
      loginForm: {
        identifier: '',
        password: ''
      },
      loginMessage: {
        type: '',
        text: ''
      }
    }
  },
  methods: {
    async handleLogin() {
      try {
        const response = await fetch('/api/users/login', {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json'
          },
          body: JSON.stringify(this.loginForm)
        })
        
        const data = await response.json()
        
        if (response.ok) {
          // 登录成功
          localStorage.setItem('token', data.token)
          localStorage.setItem('user', JSON.stringify(data.user))
          
          // 触发登录成功事件
          this.$emit('login-success', data.user)
          
          this.loginMessage.type = 'success'
          this.loginMessage.text = '登录成功！'
        } else {
          // 登录失败
          this.loginMessage.type = 'error'
          this.loginMessage.text = data || '登录失败'
        }
      } catch (error) {
        this.loginMessage.type = 'error'
        this.loginMessage.text = '网络错误，请稍后再试'
      }
    }
  }
}
</script>

<style scoped>
h2 {
  text-align: center;
  margin-bottom: 30px;
  color: #333;
}

.input-group {
  margin-bottom: 20px;
}

label {
  display: block;
  margin-bottom: 8px;
  color: #555;
  font-weight: 500;
}

input {
  width: 100%;
  padding: 12px 15px;
  border: 2px solid #e1e1e1;
  border-radius: 5px;
  font-size: 16px;
  transition: border-color 0.3s ease;
}

input:focus {
  outline: none;
  border-color: #667eea;
}

button {
  width: 100%;
  padding: 12px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border: none;
  border-radius: 5px;
  font-size: 16px;
  font-weight: 600;
  cursor: pointer;
  transition: transform 0.2s ease;
}

button:hover {
  transform: translateY(-2px);
}

button:active {
  transform: translateY(0);
}

.message {
  margin-top: 15px;
  padding: 10px;
  border-radius: 5px;
  text-align: center;
  display: none;
}

.message.success {
  background-color: #d4edda;
  color: #155724;
  border: 1px solid #c3e6cb;
  display: block;
}

.message.error {
  background-color: #f8d7da;
  color: #721c24;
  border: 1px solid #f5c6cb;
  display: block;
}
</style>
<template>
  <div class="container">
    <div class="tabs">
      <div 
        class="tab" 
        :class="{ active: activeTab === 'login' }"
        @click="switchTab('login')"
      >
        登录
      </div>
      <div 
        class="tab" 
        :class="{ active: activeTab === 'register' }"
        @click="switchTab('register')"
      >
        注册
      </div>
    </div>

    <div class="form-container">
      <!-- 登录表单 -->
      <LoginForm 
        v-show="activeTab === 'login'"
        @login-success="handleLoginSuccess"
      />

      <!-- 注册表单 -->
      <RegisterForm 
        v-show="activeTab === 'register'"
        @register-success="handleRegisterSuccess"
      />

      <!-- 用户信息展示 -->
      <UserProfile 
        v-if="isLoggedIn"
        :user="currentUser"
        @logout="handleLogout"
      />
    </div>
  </div>
</template>

<script>
import LoginForm from './auth/LoginForm.vue'
import RegisterForm from './auth/RegisterForm.vue'
import UserProfile from './auth/UserProfile.vue'

export default {
  name: 'AuthComponent',
  components: {
    LoginForm,
    RegisterForm,
    UserProfile
  },
  data() {
    return {
      activeTab: 'login',
      isLoggedIn: false,
      currentUser: null
    }
  },
  mounted() {
    // 页面加载时检查是否已登录
    this.checkLoginStatus()
  },
  methods: {
    switchTab(tabName) {
      this.activeTab = tabName
    },
    
    handleLoginSuccess(user) {
      this.currentUser = user
      this.isLoggedIn = true
    },
    
    handleRegisterSuccess(username) {
      // 注册成功后切换到登录标签
      setTimeout(() => {
        this.switchTab('login')
      }, 1000)
    },
    
    handleLogout() {
      localStorage.removeItem('token')
      localStorage.removeItem('user')
      
      this.isLoggedIn = false
      this.currentUser = null
    },
    
    checkLoginStatus() {
      const token = localStorage.getItem('token')
      const user = localStorage.getItem('user')
      
      if (token && user) {
        this.currentUser = JSON.parse(user)
        this.isLoggedIn = true
      }
    }
  }
}
</script>

<style scoped>
.container {
  background: white;
  border-radius: 10px;
  box-shadow: 0 15px 35px rgba(0, 0, 0, 0.1);
  overflow: hidden;
  width: 100%;
  max-width: 400px;
}

.tabs {
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
  color: #667eea;
  border-bottom: 3px solid #667eea;
}

.form-container {
  padding: 30px;
}
</style>