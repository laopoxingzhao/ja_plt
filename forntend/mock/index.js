// mock/index.js
const Mock = require('mockjs')

// 判断是否开启mock
const enableMock = process.env.NODE_ENV === 'development'

if (enableMock) {
  // 加载所有mock规则
  require('./user')
  require('./service')
  require('./order')
  
  console.log('Mock initialized successfully')
}

module.exports = Mock