<template>
  <div class="round">
    <code>{{ round.expression }}</code>
    <Item v-for="item in round.items" :key="item.item_id" :item="item" @setAnswer="set_answer"></Item>
    <button @click="send_answer">Next</button>
  </div>
</template>

<script>
import Item from "./Item";
export default {
  name: "Round",
  props: ["round", "game_id"],
  components: {Item},
  data: function() {
    return {
      answers: []
    }
  },
  methods: {
    send_answer: function () {
      const requestOptions = {
        method: 'POST',
        headers: {"Authorization": this.$store.state.session_token, "Content-Type": "application/json"},
        body: JSON.stringify({"game_id": this.game_id, "answers": this.answers })
      };
      console.log(this.answers)
      const self = this;
      fetch(`${process.env.VUE_APP_API_URI}/answer_round`, requestOptions)
        .then(res => res.json())
        .then(res => {
          self.$emit("nextRound", res)
        })
        .catch(err => console.log(err))
    },
    set_answer: function (item_id, answer) {
      let last_response_index = this.answers.findIndex(el => el.item_id === item_id)
      if (last_response_index !== -1) {
        this.answers[last_response_index].answer = answer
      } else {
        this.answers.push({"item_id": item_id, "answer": answer})
      }
    }
  }
}
</script>

<style scoped>

</style>
