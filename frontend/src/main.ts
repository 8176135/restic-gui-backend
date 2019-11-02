import Vue from "vue"
import App from "./App.vue"
import VueNativeSock from "vue-native-websocket"

Vue.config.productionTip = false

Vue.use(VueNativeSock, "ws://localhost:3012", {
  reconnection: true, // (Boolean) whether to reconnect automatically (false)
  reconnectionAttempts: 5, // (Number) number of reconnection attempts before giving up (Infinity),
  reconnectionDelay: 3000, // (Number) how long to initially wait before attempting a new (1000)
  format: "json"
})

new Vue({
  render: h => h(App)
}).$mount("#app")
