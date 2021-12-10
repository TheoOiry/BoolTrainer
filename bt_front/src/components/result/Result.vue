<template>
  <div>
  <div class="boxShadow">
    <div class="containerResult">
      <div class="resultLine">
        Number of errors
        <span>{{ this.results.score.nb_errors }}</span>
      </div>
      <div class="resultLine">
        Number of items
        <span>{{ this.results.score.nb_items }}</span>
      </div>
      <div class="resultLine">
        Time spend
        <span>{{ this.results.score.time_spend }}</span>
      </div>
    </div>
  </div>


    <div class="boxShadow">
    <div class="roundContainer">
      <div v-for="(round, index) in results.rounds" :key="round.round_id">
        <button class="roundButton" @click="current_showed_round = round">Round {{ index + 1 }}</button>
      </div>
    </div>
    </div>
      <div class="boxShadow">
    <RoundResult :round="current_showed_round" />
        </div>

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
.containerResult{
  display: flex;
}

.resultLine{
  display: flex;
  margin:0 10px;
  flex-direction: column;
  align-items: center;
  font-family: 'Roboto', sans-serif;
}

.resultLine span{
  font-size: 25px;
  font-family: 'Roboto', sans-serif;
  margin-top: 5px;
}

.roundContainer{
  display: flex;
  flex-wrap: wrap;
  justify-content: center;
}

.roundButton{
  margin: 8px;
}
</style>
