<template>
  <div id="app">
    <img alt="Vue logo" src="./assets/logo.png">
    <ul id="demo">
      <ListItem
        class="item"
        :item="treeData"
        :isFolder="true"
        @update-contents="updateContents"
      ></ListItem>
    </ul>
    <button @click="getdata">Get data stuff</button>
  </div>
</template>

<script lang="ts">
import { Component, Vue } from "vue-property-decorator"
import HelloWorld from "./components/HelloWorld.vue"
import ListItem from "./components/ListItem.vue"

const treeData2 = {
  name: "/"
}

@Component({
  components: {
    HelloWorld,
    ListItem
  }
})
export default class App extends Vue {
  treeData: Object = treeData2;

  updateContents(item, path) {
    this.$socket.sendObj({ type: "PathLookup", path: path })

    this.$options.sockets.onmessage = (data) => {
      delete this.$options.sockets.onmessage

      Vue.set(item, "children", JSON.parse(data.data))
    }
  }

  getdata() {
    this.$socket.sendObj({ type: "RetrieveAllConfigs" })

    this.$options.sockets.onmessage = (data) => {
      delete this.$options.sockets.onmessage

      console.log(JSON.parse(data.data))
    }
  }

  mounted() {

  }
}
</script>

<style lang="scss">
#app {
  $color: #2c3e50;
  font-family: 'Avenir', Helvetica, Arial, sans-serif;
  text-align: left;
  color: $color;
  margin-top: 60px;
  margin-left: 100px;
  margin-right: 100px;
}

.item {
  cursor: pointer;
}

.bold {
  font-weight: bold;
}

ul {
  padding-left: 1em;
  line-height: 1.5em;
  list-style-type: dot;
}

.included {
  color: #336c30;
}

.contains {
  text-decoration: underline;
}
</style>
