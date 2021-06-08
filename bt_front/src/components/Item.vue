<template>
  <div class="item">
    <Variable v-for="variable in item.variables" :key="variable.name" :variable="variable"></Variable>
    <input type="radio" v-model="answer" :value="true">true
    <input type="radio" v-model="answer" :value="false">false
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

</style>
