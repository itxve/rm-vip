<script setup lang="ts">
import { ref, onMounted } from "vue";

import { writeTextFile, readTextFile, BaseDirectory } from "@tauri-apps/api/fs";
import { invoke } from "@tauri-apps/api/tauri";
import { JyJsonFile } from "../types";

const greetMsg = ref("");
const name = ref("");
const files = ref("");

const jyFileJson = ref<JyJsonFile>();

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsg.value = await invoke("greet", { name: name.value });
  await saveConfig();
}
async function onChange(e: any) {
  console.log(" e.target.files", e.target.files[0]);
  let reader = new FileReader();
  const fname: string = e.target.files[0].name;
  if (fname.indexOf("root_meta_info") == -1) {
    alert("不是一个正确的剪映文件");
    return;
  }

  reader.readAsText(e.target.files[0]);
  reader.onload = function () {
    jyFileJson.value = JSON.parse(reader.result as string);
  };
}

async function readConfig() {
  const config = await readTextFile("app.config.json", {
    dir: BaseDirectory.AppConfig,
  });
  console.log("config", config);
}

onMounted(async () => {
  await readConfig();
});

async function saveConfig() {
  if (!jyFileJson.value) {
    alert("请选择配置文件");
    return;
  }
  await writeTextFile("app.config.json", JSON.stringify(jyFileJson.value), {
    dir: BaseDirectory.AppConfig,
  });
}
</script>

<template>
  <div class="card">
    <input
      id="greet-input"
      type="file"
      webkitdirectory
      accept=".json"
      @change="onChange($event)"
      placeholder="请选择文件剪映草稿路径"
    />
    <button type="button" @click="greet()">保存配置</button>
  </div>

  <p>{{ greetMsg }}</p>

  <textarea>{{ jyFileJson }}</textarea>

  <textarea>{{ files }}</textarea>
</template>
