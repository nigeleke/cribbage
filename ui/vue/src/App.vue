<template>
  <div>
    <Header />
    <router-view />
  </div>
</template>

<script>
import Header from './components/layout/Header.vue'

export default {
  name: 'app',
  components: {
    Header
  }
}
</script>

<style>
* {
    box-sizing: border-box;
    margin: 0;
    padding: 0;
  }

body {
  font-family: Avenir, Arial, Helvetica, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  color: ghostwhite;
  background-color: #408000;
  }

button {
  color: ghostwhite;
  background-color: rgba(255, 255, 255, 0.1);
  border-radius: 10px;
  font-size: 24px;
  text-align: center;
  padding-left: 5vw;
  padding-right: 5vw;
  padding-top: 2vh;
  padding-bottom: 2vh;
  }

</style>
