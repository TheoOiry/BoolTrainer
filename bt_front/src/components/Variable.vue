<template>
  <div class="variable">
    {{ variable.name }} = {{ variable.value }};
  </div>
</template>

<script>
export default {
  name: "Variable",
  props: ["variable"]
}
</script>

<style scoped>

</style>
