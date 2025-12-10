// mock/order.js
import Mock from 'mockjs'

// 模拟订单数据
const orders = []
for (let i = 0; i < 50; i++) {
  orders.push(Mock.mock({
    order_id: '@increment',
    service_id: '@integer(1, 30)',
    customer_id: '@integer(1, 20)',
    worker_id: '@integer(1, 10)' + '',
    service_name: '@ctitle(4, 8)服务',
    customer_name: '@cname',
    worker_name: '@cname',
    contact_phone: /^1[3-9]\d{9}$/,
    address: '@county(true) @cword(3, 5)小区 @integer(1, 20)栋@integer(1, 20)号',
    appointment_time: '@datetime',
    'status|1': ['pending', 'confirmed', 'in_progress', 'completed', 'cancelled'],
    total_amount: '@float(50, 1000, 2, 2)',
    paid_amount: '@float(0, 1000, 2, 2)',
    created_at: '@datetime',
    updated_at: '@datetime'
  }))
}

// 获取订单列表
Mock.mock('/api/orders', 'get', (options) => {
  // 解析查询参数
  const url = new URL(options.url, 'http://localhost')
  const status = url.searchParams.get('status')
  const page = parseInt(url.searchParams.get('page')) || 1
  const pageSize = parseInt(url.searchParams.get('page_size')) || 10
  
  let filteredOrders = orders
  if (status) {
    filteredOrders = orders.filter(o => o.status === status)
  }
  
  // 分页处理
  const total = filteredOrders.length
  const start = (page - 1) * pageSize
  const end = start + pageSize
  const paginatedOrders = filteredOrders.slice(start, end)
  
  return {
    code: 200,
    msg: '获取成功',
    data: {
      orders: paginatedOrders,
      total: total,
      page: page,
      page_size: pageSize
    }
  }
})

// 获取订单详情
Mock.mock(/^\/api\/orders\/\d+/, 'get', (options) => {
  const url = options.url
  const id = parseInt(url.match(/\/orders\/(\d+)/)[1])
  
  const order = orders.find(o => o.order_id === id)
  
  if (!order) {
    return {
      code: 404,
      msg: '订单不存在'
    }
  }
  
  return {
    code: 200,
    msg: '获取成功',
    data: order
  }
})

// 创建订单
Mock.mock('/api/orders', 'post', (options) => {
  const data = JSON.parse(options.body)
  
  const newOrder = Mock.mock({
    order_id: orders.length > 0 ? Math.max(...orders.map(o => o.order_id)) + 1 : 1,
    service_id: data.service_id,
    customer_id: data.customer_id,
    worker_id: data.worker_id || null,
    service_name: '@ctitle(4, 8)服务',
    customer_name: '@cname',
    worker_name: data.worker_id ? '@cname' : null,
    contact_phone: data.contact_phone,
    address: data.address,
    appointment_time: data.appointment_time,
    status: 'pending',
    total_amount: data.total_amount,
    paid_amount: 0,
    created_at: '@datetime',
    updated_at: '@datetime'
  })
  
  orders.push(newOrder)
  
  return {
    code: 201,
    msg: '订单创建成功',
    data: newOrder
  }
})

// 更新订单状态
Mock.mock(/^\/api\/orders\/\d+\/status/, 'put', (options) => {
  const url = options.url
  const id = parseInt(url.match(/\/orders\/(\d+)/)[1])
  const { status } = JSON.parse(options.body)
  
  const order = orders.find(o => o.order_id === id)
  
  if (!order) {
    return {
      code: 404,
      msg: '订单不存在'
    }
  }
  
  order.status = status
  order.updated_at = new Date().toISOString()
  
  return {
    code: 200,
    msg: '订单状态更新成功',
    data: order
  }
})

export default Mock