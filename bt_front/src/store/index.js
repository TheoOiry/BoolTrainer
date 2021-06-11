import Vue from 'vue'
import Vuex from 'vuex'

Vue.use(Vuex)

let get_token = async function () {
  let token = localStorage.getItem("session_token")
  const requestOptions = { method: 'GET', headers: {"Authorization": token } };
  const response = await fetch(`${process.env.VUE_APP_API_URI}/ping_session`, requestOptions)
  if (response.status === 200)
    return token;
  else {
    const response = await fetch(`${process.env.VUE_APP_API_URI}/create_session`, { method: 'POST' })
    return (await response.json()).jwt_token
  }
}

export default new Vuex.Store({
  state: {
    session_token: get_token(),
  },
  mutations: {
    async setToken (state) {
      const token = await get_token();
      state.session_token = token
      localStorage.setItem("session_token", token)
    },
  },
  getters: {
    auth_full_header: state => ({"Authorization": state.session_token}),
    is_logged: async state => {
      if (state.session_token === undefined) {
        state.session_token = await get_token();
      }
      return state.session_token !== null
    }
  },
})
