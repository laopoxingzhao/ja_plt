<template>
  <form class="form active" @submit.prevent="handleLogin">
    <h2>用户登录</h2>
    <div class="input-group">
      <label for="login-identifier">用户名或邮箱</label>
      <input 
        type="text" 
        id="login-identifier" 
        v-model="form.identifier"
        required
      >
    </div>
    <div class="input-group">
      <label for="login-password">密码</label>
      <input 
        type="password" 
        id="login-password" 
        v-model="form.password"
        required
      >
    </div>
    <button type="submit">登录</button>
    <div 
      id="login-message" 
      class="message"
      :class="message.type"
      v-show="message.text"
    >
      {{ message.text }}
    </div>
  </form>
</template>

<script>
import { reactive } from 'vue'

export default {
  name: 'LoginFormComposition',
  emits: ['login-success'],
  setup(props, { emit }) {
    const form = reactive({
      identifier: '',
      password: ''
    })

    const message = reactive({
      type: '',
      text: ''
    })

    const handleLogin = async () => {
      try {
        const response = await fetch('/api/users/login', {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json'
          },
          body: JSON.stringify(form)
        })
        
        const data = await response.json()
        
        if (response.ok) {
          // 登录成功
          localStorage.setItem('token', data.token)
          localStorage.setItem('user', JSON.stringify(data.user))
          
          message.type = 'success'
          message.text = '登录成功！'
          
          // 触发登录成功事件
          emit('login-success', data.user)
        } else {
          // 登录失败
          message.type = 'error'
          message.text = data || '登录失败'
        }
      } catch (error) {
        message.type = 'error'
        message.text = '网络错误，请稍后再试'
      }
    }

    return {
      form,
      message,
      handleLogin
    }
  }
}
</script>

<style scoped>
.form {
  display: none;
}

.form.active {
  display: block;
}

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