<template>
  <div class="home">
    <div class="homeContainer">
      <h1>Bool trainer</h1>
      <p>The first training application for boolean expressions.</p>
      <router-link to="/game">
        <button id="buttonStart" class="grow">Start !</button>
      </router-link>
    </div>
  </div>
</template>

<script>

export default {
  name: 'Home',
}
</script>

<style scoped>
.home {
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  height: 100vh;
}
p {
  margin-top: 0;
  font-family: 'Roboto', sans-serif;
  font-size: 15px;
}
.homeContainer {
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  align-items: center;
}
</style>
