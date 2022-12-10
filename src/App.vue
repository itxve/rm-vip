<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import RmMain from "./components/RmMain.vue";
import { version } from "../package.json";
import { NNotificationProvider } from "naive-ui";
</script>

<template>
  <n-notification-provider>
    <div class="container">
      <RmMain />
      <footer class="footer">
        <div>
          © 2022
          <a target="_blank" href="https://github.com/itxve/rm-vip">
            <b>GitHub</b>
          </a>
          V{{ version }}
        </div>
      </footer>
    </div>
  </n-notification-provider>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
