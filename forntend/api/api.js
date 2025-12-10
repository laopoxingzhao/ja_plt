// api/api.js
// 简单的Mock数据实现
const mockData = {
  // 服务分类Mock数据
  '/services/categories': [
    { id: 1, name: '保洁清洗', icon: '/static/clean.png' },
    { id: 2, name: '搬家服务', icon: '/static/move.png' },
    { id: 3, name: '维修安装', icon: '/static/repair.png' },
    { id: 4, name: '保姆月嫂', icon: '/static/nanny.png' },
    { id: 5, name: '管道疏通', icon: '/static/plumbing.png' },
    { id: 6, name: '家电清洗', icon: '/static/appliance.png' },
    { id: 7, name: '家具保养', icon: '/static/furniture.png' },
    { id: 8, name: '全部服务', icon: '/static/all.png' }
  ],
  
  // 服务列表Mock数据
  '/services': Array.from({ length: 30 }, (_, i) => ({
    service_id: i + 1,
    service_name: ['家庭保洁', '深度清洁', '空调清洗', '油烟机清洗', '地板打蜡'][Math.floor(Math.random() * 5)],
    description: ['专业的家庭保洁服务，让您家居环境整洁舒适', '深度清洁服务，彻底清除家中污垢', '专业空调清洗，提高制冷效果'][Math.floor(Math.random() * 3)],
    base_price: Math.floor(Math.random() * 500) + 50,
    category_id: Math.floor(Math.random() * 8) + 1,
    image: `/static/service-${(i % 3) + 1}.jpg`,
    duration: Math.floor(Math.random() * 4) + 1,
    status: 'active',
    created_at: new Date().toISOString(),
    updated_at: new Date().toISOString()
  })),
  
  // 订单列表Mock数据
  '/orders': Array.from({ length: 20 }, (_, i) => ({
    order_id: i + 1,
    service_id: Math.floor(Math.random() * 30) + 1,
    customer_id: 1,
    worker_id: Math.floor(Math.random() * 10) + 1,
    service_name: ['家庭保洁', '深度清洁', '空调清洗'][Math.floor(Math.random() * 3)],
    customer_name: `客户${i + 1}`,
    worker_name: `技师${Math.floor(Math.random() * 10) + 1}`,
    contact_phone: `138${Math.floor(Math.random() * 10000000).toString().padStart(8, '0')}`,
    address: `北京市朝阳区某某小区${Math.floor(Math.random() * 20) + 1}号楼${Math.floor(Math.random() * 1000) + 1}室`,
    appointment_time: new Date(Date.now() + Math.random() * 7 * 24 * 60 * 60 * 1000).toISOString(),
    status: ['pending', 'confirmed', 'in_progress', 'completed', 'cancelled'][Math.floor(Math.random() * 5)],
    total_amount: Math.floor(Math.random() * 1000) + 50,
    paid_amount: Math.floor(Math.random() * 1000) + 50,
    created_at: new Date().toISOString(),
    updated_at: new Date().toISOString()
  }))
}

// 判断是否启用Mock
const enableMock = process.env.NODE_ENV === 'development' && process.env.VUE_APP_MOCK === 'true'

// API基础URL
const BASE_URL = process.env.VUE_APP_API_BASE_URL || 'http://localhost:8000'

// 通用请求函数
function request(url, options = {}) {
  // 如果启用Mock且有对应Mock数据
  if (enableMock && mockData[url]) {
    return new Promise((resolve) => {
      // 模拟网络延迟
      setTimeout(() => {
        resolve({
          code: 200,
          msg: 'success',
          data: mockData[url]
        })
      }, 300)
    })
  }
  
  // 使用uni.request发送真实请求
  return new Promise((resolve, reject) => {
    // 添加认证token
    const token = uni.getStorageSync('token')
    const header = {
      'Content-Type': 'application/json',
      ...options.header
    }
    
    if (token) {
      header.Authorization = `Bearer ${token}`
    }
    
    uni.request({
      url: BASE_URL + url,
      header,
      ...options,
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
    // 如果启用Mock
    if (enableMock) {
      return new Promise((resolve) => {
        setTimeout(() => {
          // 简单验证
          if (data.password !== '123456') {
            resolve({
              code: 401,
              msg: '密码错误'
            })
            return
          }
          
          resolve({
            code: 200,
            msg: '登录成功',
            data: {
              token: 'mock-token-' + Date.now(),
              refresh_token: 'mock-refresh-token-' + Date.now(),
              user: {
                user_id: 1,
                username: data.identifier,
                email: 'user@example.com',
                phone: '13800138000',
                user_type: 'customer',
                avatar_url: '/static/avatar.jpg',
                real_name: '张三',
                is_verified: true,
                balance: 1000.00,
                status: 'active',
                created_at: new Date().toISOString(),
                updated_at: new Date().toISOString()
              }
            }
          })
        }, 500)
      })
    }
    
    return new Promise((resolve, reject) => {
      uni.request({
        url: BASE_URL + '/users/login',
        method: 'POST',
        header: {
          'Content-Type': 'application/json'
        },
        data,
        success: (res) => {
          if (res.statusCode >= 200 && res.statusCode < 300) {
            if (res.data.code === 200) {
              const result = res.data.data
              uni.setStorageSync('token', result.token)
              uni.setStorageSync('refreshToken', result.refresh_token)
              uni.setStorageSync('user', JSON.stringify(result.user))
              resolve(result)
            } else {
              reject(new Error(res.data.msg || '登录失败'))
            }
          } else {
            reject(new Error(res.data.msg || '登录失败'))
          }
        },
        fail: (err) => {
          reject(new Error(err.errMsg || '网络错误'))
        }
      })
    })
  },
  
  // 用户登出
  async logout() {
    const refreshToken = uni.getStorageSync('refreshToken')
    
    // 清除本地存储
    uni.removeStorageSync('token')
    uni.removeStorageSync('refreshToken')
    uni.removeStorageSync('user')
    
    // 如果启用Mock，直接返回成功
    if (enableMock) {
      return Promise.resolve()
    }
    
    // 发送登出请求
    return new Promise((resolve, reject) => {
      uni.request({
        url: BASE_URL + '/users/logout',
        method: 'POST',
        header: {
          'Content-Type': 'application/json'
        },
        data: {
          refresh_token: refreshToken
        },
        success: () => {
          resolve()
        },
        fail: (err) => {
          // 即使服务器报错也认为登出成功
          resolve()
        }
      })
    })
  },
  
  // 获取用户信息
  async getUserInfo() {
    if (enableMock) {
      return new Promise((resolve) => {
        setTimeout(() => {
          resolve({
            user_id: 1,
            username: 'mockuser',
            email: 'user@example.com',
            phone: '13800138000',
            user_type: 'customer',
            avatar_url: '/static/avatar.jpg',
            real_name: '张三',
            is_verified: true,
            balance: 1000.00,
            status: 'active',
            created_at: new Date().toISOString(),
            updated_at: new Date().toISOString()
          })
        }, 500)
      })
    }
    
    return request('/users/profile')
  },
  
  // 更新用户信息
  async updateUserInfo(data) {
    if (enableMock) {
      return new Promise((resolve) => {
        setTimeout(() => {
          resolve({
            ...data,
            user_id: 1,
            username: 'mockuser',
            email: data.email || 'user@example.com',
            phone: data.phone || '13800138000',
            updated_at: new Date().toISOString()
          })
        }, 500)
      })
    }
    
    return request('/users/profile', {
      method: 'PUT',
      data
    })
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
    // Mock处理分页参数
    if (enableMock) {
      const services = mockData['/services'] || []
      const page = parseInt(params.page) || 1
      const pageSize = parseInt(params.page_size) || 10
      const start = (page - 1) * pageSize
      const end = start + pageSize
      const paginatedServices = services.slice(start, end)
      
      return new Promise((resolve) => {
        setTimeout(() => {
          resolve({
            services: paginatedServices,
            total: services.length,
            page: page,
            page_size: pageSize
          })
        }, 300)
      })
    }
    
    let url = '/services'
    const queryParams = new URLSearchParams(params).toString()
    if (queryParams) {
      url += '?' + queryParams
    }
    return request(url)
  },
  
  // 获取服务详情
  async getServiceById(id) {
    // Mock数据处理
    if (enableMock) {
      const services = mockData['/services'] || []
      const service = services.find(s => s.service_id == id)
      if (service) {
        return Promise.resolve(service)
      } else {
        return Promise.reject(new Error('服务不存在'))
      }
    }
    
    return request(`/services/${id}`)
  }
}

// 订单相关API
export const orderApi = {
  // 获取订单列表
  async getOrders(params = {}) {
    // Mock处理订单分页参数
    if (enableMock) {
      const orders = mockData['/orders'] || []
      const page = parseInt(params.page) || 1
      const pageSize = parseInt(params.page_size) || 10
      const start = (page - 1) * pageSize
      const end = start + pageSize
      const paginatedOrders = orders.slice(start, end)
      
      return new Promise((resolve) => {
        setTimeout(() => {
          resolve({
            orders: paginatedOrders,
            total: orders.length,
            page: page,
            page_size: pageSize
          })
        }, 300)
      })
    }
    
    let url = '/orders'
    const queryParams = new URLSearchParams(params).toString()
    if (queryParams) {
      url += '?' + queryParams
    }
    return request(url)
  },
  
  // 获取订单详情
  async getOrderById(id) {
    // Mock数据处理
    if (enableMock) {
      const orders = mockData['/orders'] || []
      const order = orders.find(o => o.order_id == id)
      if (order) {
        return Promise.resolve(order)
      } else {
        return Promise.reject(new Error('订单不存在'))
      }
    }
    
    return request(`/orders/${id}`)
  },
  
  // 创建订单
  async createOrder(data) {
    if (enableMock) {
      return new Promise((resolve) => {
        setTimeout(() => {
          const newOrder = {
            order_id: Date.now(),
            ...data,
            status: 'pending',
            created_at: new Date().toISOString(),
            updated_at: new Date().toISOString()
          }
          resolve(newOrder)
        }, 500)
      })
    }
    
    return request('/orders', {
      method: 'POST',
      data
    })
  },
  
  // 更新订单状态
  async updateOrderStatus(id, status) {
    if (enableMock) {
      return new Promise((resolve) => {
        setTimeout(() => {
          resolve({
            order_id: id,
            status: status,
            updated_at: new Date().toISOString()
          })
        }, 500)
      })
    }
    
    return request(`/orders/${id}/status`, {
      method: 'PUT',
      data: { status }
    })
  }
}

export default {
  request,
  userApi,
  serviceApi,
  orderApi
}