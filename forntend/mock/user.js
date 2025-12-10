// mock/user.js
const Mock = require('mockjs')

// 模拟用户数据
const users = []
for (let i = 0; i < 20; i++) {
  users.push(Mock.mock({
    user_id: i + 1,
    username: '@first',
    email: '@email',
    phone: /^1[3-9]\d{9}$/,
    'user_type|1': ['customer', 'worker', 'admin'],
    avatar_url: Mock.Random.image('100x100', Mock.Random.color(), '#FFF', 'png', '@first'),
    real_name: '@cname',
    is_verified: '@boolean',
    balance: '@float(0, 1000, 2, 2)',
    'status|1': ['active', 'inactive', 'banned'],
    created_at: '@datetime',
    updated_at: '@datetime'
  }))
}

// 登录接口
Mock.mock('/users/login', 'post', (options) => {
  const { identifier, password } = JSON.parse(options.body)
  
  // 模拟登录验证
  const user = users.find(u => 
    u.username === identifier || 
    u.email === identifier
  )
  
  if (!user) {
    return {
      code: 401,
      msg: '用户不存在'
    }
  }
  
  if (password !== '123456') {
    return {
      code: 401,
      msg: '密码错误'
    }
  }
  
  return {
    code: 200,
    msg: '登录成功',
    data: {
      token: '@guid',
      refresh_token: '@guid',
      user: user
    }
  }
})

// 注册接口
Mock.mock('/users/register', 'post', (options) => {
  const { username, email, phone, password } = JSON.parse(options.body)
  
  // 检查是否已存在
  const exists = users.some(u => 
    u.username === username || 
    u.email === email || 
    u.phone === phone
  )
  
  if (exists) {
    return {
      code: 400,
      msg: '用户名、邮箱或手机号已被注册'
    }
  }
  
  // 创建新用户
  const newUser = Mock.mock({
    user_id: users.length + 1,
    username: username,
    email: email,
    phone: phone,
    'user_type|1': ['customer', 'worker'],
    avatar_url: null,
    real_name: null,
    is_verified: false,
    balance: 0,
    status: 'active',
    created_at: '@datetime',
    updated_at: '@datetime'
  })
  
  users.push(newUser)
  
  return {
    code: 201,
    msg: '注册成功',
    data: {
      user: newUser
    }
  }
})

// 获取用户列表
Mock.mock('/users', 'get', () => {
  return {
    code: 200,
    msg: '获取成功',
    data: users
  }
})

// 获取用户详情
Mock.mock(/^\/users\/\d+/, 'get', (options) => {
  const url = options.url
  const id = parseInt(url.match(/\/users\/(\d+)/)[1])
  
  const user = users.find(u => u.user_id === id)
  
  if (!user) {
    return {
      code: 404,
      msg: '用户不存在'
    }
  }
  
  return {
    code: 200,
    msg: '获取成功',
    data: user
  }
})