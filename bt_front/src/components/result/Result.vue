<template>
  <div class="result">
    <p>Number of errors: {{ this.results.score.nb_errors }}</p>
    <p>Number of items: {{ this.results.score.nb_items }}</p>
    <p>Time spend: {{ this.results.score.time_spend }}</p>

    <ul>
      <li v-for="(round, index) in results.rounds" :key="round.round_id">
        <button @click="current_showed_round = round">Round {{ index + 1 }}</button>
      </li>
    </ul>
    <RoundResult :round="current_showed_round" />
  </div>
</template>

<script>
import RoundResult from "./RoundResult";
export default {
  name: "Result",
  components: {RoundResult},
  props: ["results"],
  data: function () {
    return {
      current_showed_round: this.results.rounds[0]
    }
  }
}
</script>

<style scoped>

</style>
