<template>
  <div>
    <div class="columns">
      <table class="table column col-8 col-mx-auto table-striped table-hover">
        <thead>
          <tr class="columns">
            <th class="column col-2">Configuration Name</th>
            <th class="column col-6">Repository Path</th>
            <th class="column col-2">Last Backup Run</th>
            <th class="column col-2">Commands</th>
          </tr>
        </thead>
        <tbody>
          <tr class="columns" v-for="(child, idx) in allConfigs" :key="idx">
            <td class="column col-2">{{child.name}}</td>
            <td class="column col-6"><b>{{configPathMapping(child).typeName}}:</b> <code>{{configPathMapping(child).path}}</code></td>
            <td class="column col-2">{{child.last_backup_time}}</td>
            <td class="column col-2 btn-group-block btn-group">
              <button class="btn btn-sm" @click="openDetails(child, idx)">Details</button>
              <button class="btn btn-sm" @click="openTargets(child, idx)">Targets</button>
              <button class="btn btn-sm btn-error" @click="openDelete(child, idx)">Delete</button>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
    <div>
      <button class="btn" @click="refreshConfigs">Refresh</button>
      <button class="btn" @click="addConfig" v-if="allConfigs !== null">Add New Config</button>
    </div>
    <div class="modal" :class="{active: showDetailsModal}" v-if="currentConfig !== null">
      <a @click="discardDetails" class="modal-overlay" aria-label="Close"></a>
      <div class="modal-container">
        <div class="modal-header">
          <a @click="discardDetails" class="btn btn-clear float-right" aria-label="Close"></a>
          <div class="modal-title h5">{{currentConfig.name}} Details</div>
        </div>
        <div class="modal-body">
          <!-- Data here -->
          <form class="content form-horizontal">
            <div class="form-group">
              <label class="form-label col-3" for="backupNameInput">Config name: </label>
              <input class="form-input col-9" type="text" id="backupNameInput" name="backupInterval" v-model="currentConfig.name">
            </div>
            <div class="form-group">
              <label class="form-label col-3" for="backupIntervalInput">Backup interval: </label>
              <input class="form-input col-8" type="text" id="backupIntervalInput" name="backupInterval" v-model="currentConfig.backup_interval">
              <label class="form-label col-1" for="backupIntervalInput">Hours</label>
            </div>
            <div class="form-group">
              <label class="form-label col-3" for="repoPathTypeInput">Repository Path: </label>
              <input class="form-input col-9 codeInput" type="text" id="repoPathJsonInput" name="repoPathJson" v-model="currentConfig.repo_path">
            </div>
            <div class="form-group">
              <label class="form-label col-3" for="repoPathTypeInput">Password: </label>
              <input class="form-input col-9 codeInput" type="password" id="repoPathJsonInput" name="repoPathJson" placeholder="<unchanged>" v-model="currentConfig.repo_password">
            </div>
            <div class="accordion">
              <input type="checkbox" id="accordion-1" name="accordion-checkbox" hidden>
              <label class="accordion-header c-hand btn" for="accordion-1">
                Forget rate config:
              </label>
              <div class="accordion-body columns">
                <div class="form-horizontal column col-10 col-ml-auto">
                  <div class="form-group">
                    <label class="form-label col-5">Keep last:</label>
                    <input class="form-input col-7" type="number" v-model.number="currentConfig.forget_rate.keep_last">
                  </div>
                  <div class="form-group">
                    <label class="form-label col-5">Keep hourly:</label>
                    <input class="form-input col-7" type="number" v-model.number="currentConfig.forget_rate.keep_hourly">
                  </div>
                  <div class="form-group">
                    <label class="form-label col-5">Keep daily:</label>
                    <input class="form-input col-7" type="number" v-model.number="currentConfig.forget_rate.keep_daily">
                  </div>
                  <div class="form-group">
                    <label class="form-label col-5">Keep weekly:</label>
                    <input class="form-input col-7" type="number" v-model.number="currentConfig.forget_rate.keep_weekly">
                  </div>
                  <div class="form-group">
                    <label class="form-label col-5">Keep monthly:</label>
                    <input class="form-input col-7" type="number" v-model.number="currentConfig.forget_rate.keep_monthly">
                  </div>
                  <div class="form-group">
                    <label class="form-label col-5">Keep yearly:</label>
                    <input class="form-input col-7" type="number" v-model.number="currentConfig.forget_rate.keep_yearly">
                  </div>
                  <!-- TODO: Other forget options -->
                </div>
              </div>
            </div>
          </form>
        </div>
        <div class="modal-footer">
          <div class="btn-group btn-group-block">
            <button @click="discardDetails" class="btn btn-lg">Discard</button>
            <button @click="saveDetails" class="btn btn-primary btn-lg">Save</button>
          </div>
        </div>
      </div>
    </div>
    <div class="modal" :class="{active: showDeleteModal}" v-if="currentConfig !== null">
      <a @click="discardDelete" class="modal-overlay" aria-label="Close"></a>
      <div class="modal-container">
        <div class="modal-header">
          <a @click="discardDelete" class="btn btn-clear float-right" aria-label="Close"></a>
          <div class="modal-title h5">Delete {{currentConfig.name}}?</div>
        </div>
        <div class="modal-body">
        </div>
        <div class="modal-footer">
          <div class="btn-group btn-group-block">
            <button @click="discardDelete" class="btn btn-lg">Discard</button>
            <button @click="deleteConfig" class="btn btn-primary btn-lg btn-error">Confirm Delete</button>
          </div>
        </div>
      </div>
    </div>
    <div class="modal" :class="{active: showTargetsModal}" v-if="currentConfig !== null">
      <a @click="discardTargets" class="modal-overlay" aria-label="Close"></a>
      <div class="modal-container">
        <div class="modal-header">
          <a @click="discardTargets" class="btn btn-clear float-right" aria-label="Close"></a>
          <div class="modal-title h5">{{currentConfig.name}} Targets</div>
        </div>
        <div class="modal-body">
          <div>
            <h5 class="columns" style="line-height: 33px">Targets: <button class="btn col-1 btn-success col-ml-auto" @click="addEntry(currentConfig.targets.folders)">Add</button></h5>
            <div class="columns" v-for="(target, idx) in currentConfig.targets.folders" :key="idx">
              <input type="text" name="target" v-model="currentConfig.targets.folders[idx]" class="form-input col-11">
              <button class="btn col-1 btn-error" @click="deleteEntry(currentConfig.targets.folders, idx)">Del</button>
            </div>
          </div>
          <hr style="margin: 25px 0;">
          <div>
            <h5 class="columns" style="line-height: 33px">Exclusions: <button class="btn col-1 btn-success col-ml-auto" @click="addEntry(currentConfig.targets.exclusions)">Add</button></h5>
            <div class="columns" v-for="(target, idx) in currentConfig.targets.exclusions" :key="idx">
              <input type="text" name="target" v-model="currentConfig.targets.exclusions[idx]" class="form-input col-11">
              <button class="btn col-1 btn-error" @click="deleteEntry(currentConfig.targets.exclusions, idx)">Del</button>
            </div>
          </div>
          <hr style="margin: 25px 0;">
          <PreviewElems :targets="currentConfig.targets"></PreviewElems>
          <hr style="margin: 25px 0;">
          <div>
            <h5 class="columns" style="line-height: 33px">Tags: <button class="btn col-1 btn-success col-ml-auto" @click="addEntry(currentConfig.targets.tags)">Add</button></h5>
            <div class="columns" v-for="(target, idx) in currentConfig.targets.tags" :key="idx">
              <input type="text" name="target" v-model="currentConfig.targets.tags[idx]" class="form-input col-11">
              <button class="btn col-1 btn-error" @click="deleteEntry(currentConfig.targets.tags, idx)">Del</button>
            </div>
          </div>
        </div>
        <div class="modal-footer">
          <div class="btn-group btn-group-block">
            <button @click="discardTargets" class="btn btn-lg">Discard</button>
            <button @click="saveDetails" class="btn btn-primary btn-lg">Save</button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
<script lang="ts">
import Component from "vue-class-component"
import { Prop, Vue, Watch } from "vue-property-decorator"
import { ConfigItem, BetterConfigItem } from "../interfaces"
import PreviewElems from "./PreviewElems.vue"

@Component({
  components: {
    PreviewElems
  }
})
export default class AllConfigs extends Vue {
  allConfigs: BetterConfigItem[] | null = null
  showDetailsModal: boolean = false
  showTargetsModal: boolean = false
  showDeleteModal: boolean = false
  currentConfig: BetterConfigItem | null = null
  currentIndex: number = 0

  refreshConfigs() {
    // @ts-ignore
    this.$socket.sendObj({ type: "RetrieveAllConfigs" })
    // @ts-ignore
    this.$options.sockets.onmessage = (data: any) => {
      // @ts-ignore
      delete this.$options.sockets.onmessage

      let parsedServerData = JSON.parse(data.data)
      console.log(parsedServerData)

      this.allConfigs = parsedServerData
    }
  }

  configPathMapping(child: BetterConfigItem) {
    let parsed = JSON.parse(child.repo_path)
    let typeName = Object.keys(parsed)[0]
    return { typeName: typeName, path: parsed[typeName] }
  }

  openDetails(child: BetterConfigItem, currentIndex: number) {
    this.showDetailsModal = true
    this.currentIndex = currentIndex
    this.currentConfig = JSON.parse(JSON.stringify(child))
  }

  openTargets(child: BetterConfigItem, currentIndex: number) {
    this.showTargetsModal = true
    this.currentIndex = currentIndex
    this.currentConfig = JSON.parse(JSON.stringify(child))
  }

  openDelete(child: BetterConfigItem, currentIndex: number) {
    this.showDeleteModal = true
    this.currentIndex = currentIndex
    this.currentConfig = JSON.parse(JSON.stringify(child))
  }

  saveDetails() {
    if (this.currentConfig === null || this.allConfigs === null) {
      console.log("ERROR")
      alert("Error, save details failed")
      return
    }

    if (this.checkDupName(this.currentConfig.name, this.currentIndex)) {
      return
    }
    this.allConfigs[this.currentIndex] = this.currentConfig
    console.log(JSON.stringify({ type: "SaveAllConfigs", config: this.allConfigs }))
    // @ts-ignore
    this.$socket.sendObj({ type: "SaveAllConfigs", config: this.allConfigs })
    this.discardDetails()
    this.discardTargets()
  }

  checkDupName(candi: string, idx: number) {
    if (this.allConfigs === null) {
      alert("all configs is null in check dup name")
      return true
    }
    for (let i = 0; i < this.allConfigs.length; ++i) {
      if (this.allConfigs[i].name === candi && i !== idx) {
        return true
      }
    }

    return false
  }

  addConfig() {
    if (this.allConfigs === null) {
      alert("Add configs failed, all confgis null")
      return
    }

    // @ts-ignore
    this.$socket.sendObj({ type: "GetNewTemplate" })
    // @ts-ignore
    this.$options.sockets.onmessage = (data: any) => {
      // @ts-ignore
      delete this.$options.sockets.onmessage
      console.log(data.data)
      let parsedServerData = JSON.parse(data.data)
      console.log(parsedServerData)

      this.allConfigs.push(parsedServerData)
    }
  }

  deleteConfig() {
    if (this.allConfigs === null) {
      alert("Delete configs failed, all confgis null")
      return
    }
    this.allConfigs.splice(this.currentIndex, 1)
    // @ts-ignore
    this.$socket.sendObj({ type: "SaveAllConfigs", config: this.allConfigs })
    this.discardDelete()
  }

  discardDetails() {
    this.showDetailsModal = false
    this.currentConfig = null
  }

  discardTargets() {
    this.showTargetsModal = false
    this.currentConfig = null
  }

  discardDelete() {
    this.showDeleteModal = false
  }

  addEntry(array: string[]) {
    array.push("")
  }

  deleteEntry(array: string[], idx: number) {
    array.splice(idx, 1)
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
