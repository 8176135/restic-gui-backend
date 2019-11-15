<template>
  <div>
    <ListItem :item="{ name: '/' }" :isFolder="true" @update-contents="updateContents"></ListItem>
  </div>
</template>
<script lang="ts">
import Component from "vue-class-component"
import { Prop, Vue, Watch } from "vue-property-decorator"
import { Targets, FileItem } from "../interfaces"
import ListItem from "./ListItem.vue"

@Component({
  components: {
    ListItem
  }
})
export default class PreviewElems extends Vue {
  @Prop({ required: true }) targets!: Targets

  updateContents(item: FileItem, path: string) {
    // @ts-ignore
    this.$socket.sendObj({ type: "PathLookup", path: path, target: this.targets })
    // @ts-ignore
    this.$options.sockets.onmessage = (data) => {
      // @ts-ignore
      delete this.$options.sockets.onmessage

      Vue.set(item, "children", JSON.parse(data.data))
      console.log(item)
    }
  }
}

</script>
<style lang="scss">
.configLine {
  border: solid medium black;
}

input.codeInput {
  font-family: monospace;
}

</style>
