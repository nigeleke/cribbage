<template>
  <div class="home">
    <div class="button-container">
      <button v-on:click="showStarting()">Start Your Own Game</button>
      <button v-on:click="showJoining()">Join Someone's Game</button>
      <button v-on:click="showWatching()">Watch A Game</button>
    </div>
    <div class="home-content">
      <div id="start" v-show="isStarting"><h2>Start</h2></div>
      <div id="join" v-show="isJoining"><h2>Join</h2></div>
      <div id="watch" v-show="isWatching"><h2>Watch</h2></div>
    </div>
  </div>
</template>

<script>
import axios from 'axios';

export default {
  name: 'Home',
  data() {
    return {
      isStarting: false,
      isJoining: false,
      isWatching: false
    }
  },
  methods: {
    showStarting() {
      this.hideAll();
      this.isStarting = true;
    },
    showJoining() {
      this.hideAll();
      this.isJoining = true;
    },
    showWatching() {
      this.hideAll();
      this.isWatching = true;
    },
    hideAll() {
      this.isStarting = false;
      this.isJoining = false;
      this.isWatching = false;
    }
  },
  created() {
    axios.get('http://localhost:8080/api/v1/games')
        .then(response => console.log(response))
        .catch(error => console.log(error))
  }
}
</script>

<style scoped>
.home {
  padding-left: 5vw;
  padding-right: 5vw;
}

.home-content {
  display: flex;
  justify-content: center;
  padding-top: 5vh;
}

.button-container {
  display: flex;
  justify-content: space-around;
}

.button-container button {
  max-width: 80vw;
  min-width: 20vw;
  width: 25rem;
}
</style>