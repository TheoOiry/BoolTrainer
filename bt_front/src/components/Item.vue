<template>
  <div class="item">
    <div class="variableContainer">
      <Variable v-for="variable in item.variables" :key="variable.name" :variable="variable"></Variable>
    </div>
    <div class="inputsContainer">
      <div class="inputGroup">
        <label for="trueInput">True</label>
        <input type="radio" v-model="answer" :value="true">
      </div>
      <div class="inputGroup">
        <label for="falseInput">False</label>
        <input type="radio" v-model="answer" :value="false">
      </div>
    </div>
  </div>
</template>

<script>
import Variable from "./Variable";
export default {
  name: "Item",
  components: {Variable},
  props: ["item"],
  data: function () {
    return {
      answer: null,
    }
  },
  watch: {
    answer: function (val) {
      this.$emit("setAnswer", this.item.item_id, val)
    }
  },
}
</script>

<style scoped>
.item{
  display:flex;
  flex-direction: column;
  justify-content: space-around;
}

.variableContainer{
  display: flex;
  margin: 10px 0;
}


.variableContainer > *{
  margin-right:10px;
}

.inputGroup{
  display: flex;
  align-items: center;

}

.inputsContainer{
  display: flex;
}
</style>
