// mock/service.js
const Mock = require('mockjs')

// 模拟服务分类数据
const categories = [
  { id: 1, name: '保洁清洗', icon: '/static/clean.png' },
  { id: 2, name: '搬家服务', icon: '/static/move.png' },
  { id: 3, name: '维修安装', icon: '/static/repair.png' },
  { id: 4, name: '保姆月嫂', icon: '/static/nanny.png' },
  { id: 5, name: '管道疏通', icon: '/static/plumbing.png' },
  { id: 6, name: '家电清洗', icon: '/static/appliance.png' },
  { id: 7, name: '家具保养', icon: '/static/furniture.png' },
  { id: 8, name: '全部服务', icon: '/static/all.png' }
]

// 模拟服务数据
const services = []
for (let i = 0; i < 30; i++) {
  services.push(Mock.mock({
    service_id: i + 1,
    service_name: '@ctitle(4, 8)服务',
    description: '@cparagraph(1)',
    base_price: '@float(30, 500, 2, 2)',
    category_id: '@integer(1, 8)',
    image: Mock.Random.image('200x200', Mock.Random.color(), '#FFF', 'png', '@ctitle(2, 4)'),
    duration: '@integer(1, 4)',
    'status|1': ['active', 'inactive'],
    created_at: '@datetime',
    updated_at: '@datetime'
  }))
}

// 获取服务分类
Mock.mock('/services/categories', 'get', () => {
  return {
    code: 200,
    msg: '获取成功',
    data: categories
  }
})

// 获取服务列表
Mock.mock('/services', 'get', (options) => {
  // 解析查询参数
  const url = new URL(options.url, 'http://localhost')
  const categoryId = url.searchParams.get('category_id')
  const page = parseInt(url.searchParams.get('page')) || 1
  const pageSize = parseInt(url.searchParams.get('page_size')) || 10
  
  let filteredServices = services
  if (categoryId) {
    filteredServices = services.filter(s => s.category_id == categoryId)
  }
  
  // 分页处理
  const total = filteredServices.length
  const start = (page - 1) * pageSize
  const end = start + pageSize
  const paginatedServices = filteredServices.slice(start, end)
  
  return {
    code: 200,
    msg: '获取成功',
    data: {
      services: paginatedServices,
      total: total,
      page: page,
      page_size: pageSize
    }
  }
})

// 获取服务详情
Mock.mock(/^\/services\/\d+/, 'get', (options) => {
  const url = options.url
  const id = parseInt(url.match(/\/services\/(\d+)/)[1])
  
  const service = services.find(s => s.service_id === id)
  
  if (!service) {
    return {
      code: 404,
      msg: '服务不存在'
    }
  }
  
  return {
    code: 200,
    msg: '获取成功',
    data: service
  }
})