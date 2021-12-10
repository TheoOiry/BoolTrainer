<template>
  <div class="item">
    <div class="variableContainer">
      <Variable v-for="variable in item.variables" :key="variable.name" :variable="variable"></Variable>
    </div>
    <span :class="item.expected && 'green'">
      <input type="radio" v-model="item.answer" disabled :value="true">true
    </span>
    <span :class="!item.expected && 'green'">
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

.variableContainer{
  display: flex;
  margin: 10px 0;
}


.variableContainer > *{
  margin-right:10px;
}

</style>
