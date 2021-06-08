<template>
  <div class="game">
    <div v-if="game_result">
      <Result :results="game_result" />
    </div>
    <div v-else-if="current_round">
      <Round :game_id="game_id" :round="current_round" @nextRound="next_round" />
    </div>
  </div>
</template>

<script>

import Round from "../components/Round";
import Result from "../components/result/Result";
export default {
  name: "Game",
  components: {Round, Result},
  data: function () {
    return {
      game_id: null,
      current_round: null,
      game_result: null,
    }
  },
  methods: {
    load_game: function() {
      const requestOptions = {
        method: 'POST',
        headers: this.$store.getters.auth_full_header,
      };
      const self = this;
      fetch(`${process.env.VUE_APP_API_URI}/create_game`, requestOptions)
        .then(res => res.json())
        .then(res => {
          self.game_id = res.game_id
          self.current_round = res.first_round
        })
        .catch(err => console.log(err))
    },
    next_round: function (res) {
      if("score" in res) {
        this.game_result = res
      } else {
        this.current_round = res.next_round;
      }
    }
  },
  async created() {
    if (!this.$store.getters.is_logged) {
      await this.$router.push("Home")
      return
    }
    await this.load_game()
  }
}

</script>

<style scoped>

</style>
