// api/api.js
// 初始化MockJS（仅在开发环境下）
if (process.env.NODE_ENV === 'development') {
  require('../mock')
}

const BASE_URL = 'http://localhost:8000'

// 请求封装
async function request(url, options = {}) {
  const token = uni.getStorageSync('token')
  
  const config = {
    url: BASE_URL + url,
    header: {
      'Content-Type': 'application/json',
      ...options.header
    },
    ...options
  }
  
  // 添加认证头
  if (token) {
    config.header.Authorization = `Bearer ${token}`
  }
  
  return new Promise((resolve, reject) => {
    uni.request({
      ...config,
      success: (res) => {
        // 检查是否有响应数据
        if (!res.data) {
          reject(new Error('服务器无响应'))
          return
        }
        
        // 如果是成功的响应
        if (res.statusCode >= 200 && res.statusCode < 300) {
          // 如果有code字段且不是成功状态
          if (res.data.code && res.data.code !== 200 && res.data.code !== 201) {
            reject(new Error(res.data.msg || '请求失败'))
            return
          }
          
          // 成功响应
          resolve(res.data.data || res.data)
        } else {
          // 错误响应
          reject(new Error(res.data.msg || '请求失败'))
        }
      },
      fail: (err) => {
        reject(new Error(err.errMsg || '网络错误'))
      }
    })
  })
}

// 用户相关API
export const userApi = {
  // 用户登录
  async login(data) {
    const response = await uni.request({
      url: BASE_URL + '/users/login',
      method: 'POST',
      header: {
        'Content-Type': 'application/json'
      },
      data
    })
    
    // 检查是否有响应数据
    if (!response.data) {
      throw new Error('服务器无响应')
    }
    
    // 如果是成功的响应
    if (response.statusCode >= 200 && response.statusCode < 300) {
      // 如果有code字段且不是成功状态
      if (response.data.code && response.data.code !== 200) {
        throw new Error(response.data.msg || '登录失败')
      }
      
      const data = response.data.data || response.data
      uni.setStorageSync('token', data.token)
      uni.setStorageSync('refreshToken', data.refresh_token)
      uni.setStorageSync('user', JSON.stringify(data.user))
      return data
    } else {
      // 错误响应
      throw new Error(response.data.msg || '登录失败')
    }
  },
  
  // 用户登出
  async logout() {
    const refreshToken = uni.getStorageSync('refreshToken')
    
    try {
      await uni.request({
        url: BASE_URL + '/users/logout',
        method: 'POST',
        header: {
          'Content-Type': 'application/json'
        },
        data: {
          refresh_token: refreshToken
        }
      })
    } catch (err) {
      console.error('登出请求失败:', err)
    } finally {
      // 无论请求成功与否，都要清除本地存储
      uni.removeStorageSync('token')
      uni.removeStorageSync('refreshToken')
      uni.removeStorageSync('user')
    }
  },
  
  // 用户注册
  async register(data) {
    const response = await uni.request({
      url: BASE_URL + '/users/register',
      method: 'POST',
      header: {
        'Content-Type': 'application/json'
      },
      data
    })
    
    // 检查是否有响应数据
    if (!response.data) {
      throw new Error('服务器无响应')
    }
    
    // 如果是成功的响应
    if (response.statusCode >= 200 && response.statusCode < 300) {
      // 如果有code字段且不是成功状态
      if (response.data.code && response.data.code !== 200 && response.data.code !== 201) {
        throw new Error(response.data.msg || '注册失败')
      }
      
      return response.data.data || response.data
    } else {
      // 错误响应
      throw new Error(response.data.msg || '注册失败')
    }
  },
  
  // 获取用户列表
  async getUsers() {
    return request('/users')
  },
  
  // 获取用户详情
  async getUserById(id) {
    return request(`/users/${id}`)
  }
}

// 服务相关API
export const serviceApi = {
  // 获取服务分类
  async getCategories() {
    return request('/services/categories')
  },
  
  // 获取服务列表
  async getServices(params = {}) {
    const queryParams = new URLSearchParams(params).toString()
    const url = queryParams ? `/services?${queryParams}` : '/services'
    return request(url)
  },
  
  // 获取服务详情
  async getServiceById(id) {
    return request(`/services/${id}`)
  }
}

// 订单相关API
export const orderApi = {
  // 获取订单列表
  async getOrders(params = {}) {
    const queryParams = new URLSearchParams(params).toString()
    const url = queryParams ? `/orders?${queryParams}` : '/orders'
    return request(url)
  },
  
  // 获取订单详情
  async getOrderById(id) {
    return request(`/orders/${id}`)
  },
  
  // 创建订单
  async createOrder(data) {
    return request('/orders', {
      method: 'POST',
      data
    })
  },
  
  // 更新订单状态
  async updateOrderStatus(id, status) {
    return request(`/orders/${id}/status`, {
      method: 'PUT',
      data: { status }
    })
  }
}

export default {
  userApi,
  serviceApi,
  orderApi
}