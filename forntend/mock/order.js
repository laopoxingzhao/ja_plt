// mock/order.js
const Mock = require('mockjs')

// 模拟订单状态
const orderStatuses = ['pending', 'confirmed', 'assigned', 'ongoing', 'completed', 'cancelled']
const paymentStatuses = ['pending', 'paid', 'refunded', 'failed']

// 模拟订单数据
const orders = []
for (let i = 0; i < 50; i++) {
  const customerId = Mock.mock('@integer(1, 20)')
  const serviceId = Mock.mock('@integer(1, 30)')
  
  orders.push(Mock.mock({
    order_id: i + 1,
    customer_id: customerId,
    service_id: serviceId,
    service_date: '@date',
    time_slot: '@pick(["morning", "afternoon", "evening", "full_day"])',
    duration: '@integer(1, 4)',
    unit_price: '@float(30, 500, 2, 2)',
    subtotal: '@float(30, 2000, 2, 2)',
    discount_amount: '@float(0, 100, 2, 2)',
    total_amount: function() {
      return parseFloat((this.subtotal - this.discount_amount).toFixed(2))
    },
    'payment_status|1': paymentStatuses,
    'order_status|1': orderStatuses,
    special_instructions: '@cparagraph(1)',
    address_id: '@integer(1, 10)',
    worker_id: '@integer(1, 10)',
    created_at: '@datetime',
    updated_at: '@datetime'
  }))
}

// 获取订单列表
Mock.mock('/orders', 'get', (options) => {
  // 解析查询参数
  const url = new URL(options.url, 'http://localhost')
  const customerId = url.searchParams.get('customer_id')
  const status = url.searchParams.get('status')
  const page = parseInt(url.searchParams.get('page')) || 1
  const pageSize = parseInt(url.searchParams.get('page_size')) || 10
  
  let filteredOrders = orders
  if (customerId) {
    filteredOrders = orders.filter(o => o.customer_id == customerId)
  }
  if (status) {
    filteredOrders = filteredOrders.filter(o => o.order_status === status)
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
Mock.mock(/^\/orders\/\d+/, 'get', (options) => {
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
Mock.mock('/orders', 'post', (options) => {
  const orderData = JSON.parse(options.body)
  
  const newOrder = Mock.mock({
    order_id: orders.length + 1,
    customer_id: orderData.customer_id,
    service_id: orderData.service_id,
    service_date: orderData.service_date,
    time_slot: orderData.time_slot,
    duration: orderData.duration,
    unit_price: orderData.unit_price,
    subtotal: orderData.subtotal,
    discount_amount: orderData.discount_amount,
    total_amount: function() {
      return parseFloat((this.subtotal - this.discount_amount).toFixed(2))
    },
    payment_status: 'pending',
    order_status: 'pending',
    special_instructions: orderData.special_instructions,
    address_id: orderData.address_id,
    worker_id: null,
    created_at: '@datetime',
    updated_at: '@datetime'
  })
  
  orders.unshift(newOrder)
  
  return {
    code: 201,
    msg: '订单创建成功',
    data: newOrder
  }
})

// 更新订单状态
Mock.mock(/^\/orders\/\d+\/status/, 'put', (options) => {
  const url = options.url
  const id = parseInt(url.match(/\/orders\/(\d+)\/status/)[1])
  const { status } = JSON.parse(options.body)
  
  const order = orders.find(o => o.order_id === id)
  
  if (!order) {
    return {
      code: 404,
      msg: '订单不存在'
    }
  }
  
  order.order_status = status
  order.updated_at = Mock.mock('@datetime')
  
  return {
    code: 200,
    msg: '订单状态更新成功',
    data: order
  }
})