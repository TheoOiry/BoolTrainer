import Vue from 'vue'
import Vuex from 'vuex'

Vue.use(Vuex)

export default new Vuex.Store({
  state: {
    session_token: localStorage.getItem("session_token"),
  },
  mutations: {
    setToken (state, session_token) {
      state.session_token = session_token
      localStorage.setItem("session_token", session_token)
    },
  },
  actions: {
  },
  getters: {
    auth_full_header: state => ({"Authorization": state.session_token}),
    is_logged: state => state.session_token !== null
  },
})
