<template>
  <div class="home">
    <router-link to="/game">Start!</router-link>
  </div>
</template>

<script>

export default {
  name: 'Home',
  methods: {

  },
  beforeMount() {
    if (!this.$store.getters.is_logged) {
      const requestOptions = {
        method: 'POST'
      };

      fetch(`${process.env.VUE_APP_API_URI}/create_session`, requestOptions)
        .then(res => res.json())
        .then(res => {
          this.$store.commit("setToken", res.jwt_token)
        })
        .catch(err => console.log(err))
    }
  }
}
</script>
