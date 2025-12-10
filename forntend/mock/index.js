// mock/index.js
import Mock from 'mockjs'

// mock目录已废弃，使用api.js中的简单mock实现

// 判断是否开启mock
const enableMock = process.env.NODE_ENV === 'development' && process.env.VUE_APP_MOCK === 'true'

if (enableMock) {
  // 加载所有mock规则
  import('./user.js')
  import('./service.js')
  import('./order.js')
  
  console.log('Mock initialized successfully')
}

export default Mock