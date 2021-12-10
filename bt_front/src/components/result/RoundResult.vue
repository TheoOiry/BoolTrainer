<template>
  <div class="round_result">
    <div class="boxShadow">
      <code>{{ round.expression }}</code>
    </div>
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
.boxShadow{
  display: flex;
  flex-direction: column;
  border-radius: 10px;
  margin: 10px 0;
  padding: 20px;
  max-width: 400px;
  box-shadow: none !important;
  background-color: #383838;
  color: white;
}


</style>
