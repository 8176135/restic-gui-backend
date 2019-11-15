<template>
  <div id="app" class="container">
    <img alt="Vue logo" src="./assets/logo.png">
    <ul id="illustration">
      <AllConfigs class="allConfigs"></AllConfigs>
    </ul>
  </div>
</template>

<script lang="ts">
import { Component, Vue } from "vue-property-decorator"
import HelloWorld from "./components/HelloWorld.vue"
import ListItem from "./components/ListItem.vue"
import AllConfigs from "./components/AllConfigs.vue"
import { FileItem } from "./interfaces"
import PreviewElems from "./components/PreviewElems.vue"

@Component({
  components: {
    HelloWorld,
    ListItem,
    AllConfigs
  }
})
export default class App extends Vue {
  mounted() {

  }
}
</script>

<style lang="scss">
// Import full Spectre source code
@import "node_modules/spectre.css/src/spectre";

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

#illustration ul, #illustration {
  list-style-type: none;
  list-style: disc outside !important;
}

.included {
  color: #336c30;
}

.contains {
  text-decoration: underline;
}
</style>
