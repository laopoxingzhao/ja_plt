<template>
  <div id="user-info" class="user-info">
    <h2>用户信息</h2>
    <div id="user-details">
      <p><strong>用户ID:</strong> {{ user.user_id }}</p>
      <p><strong>用户名:</strong> {{ user.username }}</p>
      <p><strong>邮箱:</strong> {{ user.email }}</p>
      <p><strong>手机号:</strong> {{ user.phone }}</p>
      <p><strong>用户类型:</strong> {{ user.type }}</p>
      <p><strong>账户状态:</strong> {{ user.status }}</p>
    </div>
    <button class="logout-btn" @click="logout">退出登录</button>
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
  emits: ['logout'],
  methods: {
    logout() {
      // 触发登出事件
      this.$emit('logout')
    }
  }
}
</script>

<style scoped>
.user-info {
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
}
</style>