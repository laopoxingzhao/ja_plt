<template>
  <div id="app">
    <AuthComponent />
  </div>
</template>

<script>
import AuthComponent from './components/AuthComponent.vue'

export default {
  name: 'App',
  components: {
    AuthComponent
  }
}
</script>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  min-height: 100vh;
  display: flex;
  justify-content: center;
  align-items: center;
  padding: 20px;
}

#app {
  width: 100%;
  height: 100%;
}
</style>