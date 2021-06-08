<template>
  <div class="round_result">
    <code>{{ round.expression }}</code>
    <ItemResult v-for="item in round.items" :key="item.item_id" :item="item"></ItemResult>
  </div>
</template>

<script>
import ItemResult from "./ItemResult";
export default {
  name: "RoundResult",
  props: ["round"],
  components: {ItemResult}
}
</script>

<style scoped>

</style>
