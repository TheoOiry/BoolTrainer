<template>
  <div class="round">
    <div class="boxShadow">
      Expression
      <div class="boxShadowCode">
        <code>{{ round.expression }}</code>
      </div>
    </div>

    <div class="boxShadow">
      <Item v-for="item in round.items" :key="item.item_id" :item="item" @setAnswer="set_answer"></Item>
      <button class="grow" @click="send_answer">Next</button>
    </div>
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

.round{
  min-width: 500px;
}

.boxShadow > * {
  margin: 5px 0;
}

.boxShadow button{
  margin-top: 20px;
}

.boxShadowCode{
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
