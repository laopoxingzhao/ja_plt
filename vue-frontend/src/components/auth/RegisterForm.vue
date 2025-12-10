<template>
  <form class="form active" @submit.prevent="handleRegister">
    <h2>用户注册</h2>
    <div class="input-group">
      <label for="register-username">用户名</label>
      <input 
        type="text" 
        id="register-username" 
        v-model="form.username"
        required
      >
    </div>
    <div class="input-group">
      <label for="register-email">邮箱</label>
      <input 
        type="email" 
        id="register-email" 
        v-model="form.email"
        required
      >
    </div>
    <div class="input-group">
      <label for="register-phone">手机号</label>
      <input 
        type="tel" 
        id="register-phone" 
        v-model="form.phone"
        required
      >
    </div>
    <div class="input-group">
      <label for="register-password">密码</label>
      <input 
        type="password" 
        id="register-password" 
        v-model="form.password"
        required
      >
    </div>
    <button type="submit">注册</button>
    <div 
      id="register-message" 
      class="message"
      :class="message.type"
      v-show="message.text"
    >
      {{ message.text }}
    </div>
  </form>
</template>

<script>
export default {
  name: 'RegisterForm',
  data() {
    return {
      form: {
        username: '',
        email: '',
        phone: '',
        password: ''
      },
      message: {
        type: '',
        text: ''
      }
    }
  },
  emits: ['register-success'],
  methods: {
    async handleRegister() {
      try {
        const response = await fetch('/api/users/register', {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json'
          },
          body: JSON.stringify(this.form)
        })
        
        const data = await response.json()
        
        if (response.ok) {
          // 注册成功
          this.message.type = 'success'
          this.message.text = '注册成功！请登录'
          
          // 触发注册成功事件
          this.$emit('register-success', this.form.username)
        } else {
          // 注册失败
          this.message.type = 'error'
          this.message.text = data || '注册失败'
        }
      } catch (error) {
        this.message.type = 'error'
        this.message.text = '网络错误，请稍后再试'
      }
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