<template>
  <div class="home">
    <div v-if="isLoaded">
      <router-link to="/game">Start!</router-link>
    </div>
  </div>
</template>

<script>

export default {
  name: 'Home',
  data: function () {
    return {
      isLoaded: false
    }
  },
  async beforeMount() {
    await this.$store.commit("setToken")
    this.isLoaded = true;
  }
}
</script>
