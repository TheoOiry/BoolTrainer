<template>
  <div class="item">
    <Variable v-for="variable in item.variables" :key="variable.name" :variable="variable"></Variable>
    <span :class="item.expected === true ? 'green' : ''">
      <input type="radio" v-model="item.answer" disabled :value="true">true
    </span>
    <span :class="item.expected === false ? 'green' : ''">
      <input type="radio" v-model="item.answer" disabled :value="false">false
    </span>
  </div>
</template>

<script>

import Variable from "../Variable";

export default {
  name: "ItemResult",
  components: {Variable},
  props: ["item"],
}
</script>

<style scoped>
.green {
  color: green;
}
</style>
