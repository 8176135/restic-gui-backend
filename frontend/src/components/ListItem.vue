<template>
  <li>
    <div :class="{bold: isFolder, included: item.included_in_backup, contains: item.has_backup_inside}" @click="toggle">
      {{ item.name }}
      <!-- <span v-if="item.has_backup_inside"> Contains Backup Somewhere Inside</span> -->
      <span v-if="isFolder">[{{ isOpen ? '-' : '+' }}]</span>
    </div>
    <ul v-show="isOpen" v-if="isFolder">
      <ListItem class="item" v-for="(child, index) in item.children" :key="index" :item="child" :isFolder="child.is_dir" @update-contents="updateContents"></ListItem>
    </ul>
  </li>
</template>
<script lang="ts">
import Component from "vue-class-component"
import { Prop, Vue, Watch } from "vue-property-decorator"
import { FileItem } from "../interfaces"

@Component
export default class ListItem extends Vue {
  @Prop() item!: FileItem;
  @Prop() isFolder!: boolean;

  isOpen: boolean = false;

  toggle() {
    if (this.isFolder) {
      if (!this.isOpen) {
        this.$emit("update-contents", this.item, this.item.name)
      }
      this.isOpen = !this.isOpen
    }
  }

  updateContents(item: FileItem, path: string) {
    this.$emit("update-contents", item, this.item.name + "/" + path)
  }

  @Watch("item", { deep: false }) // Bit of a hack
  onPropertyChanged(value: any, oldValue: any) {
    if (oldValue.children === undefined || value.children === undefined) {
      this.isOpen = false
    }
  }
}

</script>
