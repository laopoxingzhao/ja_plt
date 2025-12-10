/**
 * 用户信息相关工具方法
 */

/**
 * 获取微信小程序用户信息
 * @returns {Promise<Object>} 用户信息对象
 */
export function getWechatUserInfo() {
  return new Promise((resolve, reject) => {
    // #ifdef MP-WEIXIN
    wx.getUserProfile({
      desc: '用于完善用户资料',
      success: (res) => {
        resolve(res.userInfo);
      },
      fail: (err) => {
        reject(new Error('获取用户信息失败: ' + err.errMsg));
      }
    });
    // #endif
    
    // #ifndef MP-WEIXIN
    reject(new Error('当前环境不支持微信用户信息获取'));
    // #endif
  });
}

/**
 * 获取系统用户信息
 * @returns {Object|null} 用户信息对象或null
 */
export function getSystemUserInfo() {
  try {
    const userStr = uni.getStorageSync('user');
    return userStr ? JSON.parse(userStr) : null;
  } catch (e) {
    console.error('解析用户信息失败:', e);
    return null;
  }
}

/**
 * 更新本地存储的用户信息
 * @param {Object} userInfo 用户信息对象
 */
export function updateLocalUserInfo(userInfo) {
  try {
    uni.setStorageSync('user', JSON.stringify(userInfo));
  } catch (e) {
    console.error('更新本地用户信息失败:', e);
  }
}

/**
 * 获取用户授权设置
 * @returns {Promise<Object>} 授权设置信息
 */
export function getUserAuthSetting() {
  return new Promise((resolve, reject) => {
    // #ifdef MP-WEIXIN
    wx.getSetting({
      success: (res) => {
        resolve(res.authSetting);
      },
      fail: (err) => {
        reject(new Error('获取授权设置失败: ' + err.errMsg));
      }
    });
    // #endif
    
    // #ifndef MP-WEIXIN
    reject(new Error('当前环境不支持获取授权设置'));
    // #endif
  });
}

/**
 * 检查是否已授权获取用户信息
 * @returns {Promise<Boolean>} 是否已授权
 */
export async function checkUserInfoAuthorization() {
  try {
    const authSetting = await getUserAuthSetting();
    return !!authSetting['scope.userInfo'];
  } catch (e) {
    console.warn('检查用户信息授权状态失败:', e);
    return false;
  }
}

export default {
  getWechatUserInfo,
  getSystemUserInfo,
  updateLocalUserInfo,
  getUserAuthSetting,
  checkUserInfoAuthorization
};