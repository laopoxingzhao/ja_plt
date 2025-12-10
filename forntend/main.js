import App from './App'

// #ifndef VUE3
import Vue from 'vue'
import './uni.promisify.adaptor'

Vue.config.productionTip = false

// 全局工具函数
Vue.prototype.$utils = {
  // 格式化日期
  formatDate(date, fmt = 'YYYY-MM-DD HH:mm') {
    const d = new Date(date)
    const year = d.getFullYear()
    const month = String(d.getMonth() + 1).padStart(2, '0')
    const day = String(d.getDate()).padStart(2, '0')
    const hour = String(d.getHours()).padStart(2, '0')
    const minute = String(d.getMinutes()).padStart(2, '0')
    
    return `${year}-${month}-${day} ${hour}:${minute}`
  },
  
  // 验证手机号
  validatePhone(phone) {
    return /^1[3-9]\d{9}$/.test(phone)
  },
  
  // 格式化金额
  formatPrice(price) {
    return `¥${parseFloat(price).toFixed(2)}`
  }
}

App.mpType = 'app'
const app = new Vue({
  ...App
})
app.$mount()
// #endif

// #ifdef VUE3
import { createSSRApp } from 'vue'
import * as Pinia from 'pinia'
export function createApp() {
  const app = createSSRApp(App)
  app.use(Pinia.createPinia());
  return {
    app
  }
}
// #endif