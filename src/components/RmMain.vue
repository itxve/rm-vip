<script setup lang="ts">
import { ref, onMounted, computed } from "vue";
import useInterval from "@/hooks/useInterval";
import { invoke } from "@tauri-apps/api/tauri";
import {
  JY_FILE_NAME,
  readConfig,
  writeConfig,
  initAppDataPath,
  exsitAppConfigDir,
  sleep,
} from "@/util";
import { RustCallResult } from "@/types";
import DraftItemVue, {
  DarftProps,
  DarftItemProps,
} from "@/components/DraftItem.vue";
import { NButton, NGrid, NGi, NInput } from "naive-ui";

const fileSelectRef = ref<HTMLInputElement>();
const projectsRef = ref<DarftItemProps[]>([]);
const searchNameRef = ref();

async function onChange(e: any) {
  let reader = new FileReader();
  const fname: string = e.target.files[0].name;
  if (fname.indexOf(JY_FILE_NAME) == -1) {
    alert("不是一个正确的剪映文件");
    return;
  }
  reader.readAsText(e.target.files[0]);
  reader.onload = async function () {
    const result = reader.result as string;
    if (!result) {
      alert("请选择配置文件");
      return;
    }
    const darftRef: DarftProps = JSON.parse(result);
    await writeConfig(JSON.stringify({ root_path: darftRef.root_path }));
    // 保存后重新加载
    await loadAllProjects();
  };
}

async function loadAllProjects() {
  const exsitConfigDir = await exsitAppConfigDir();

  if (!exsitConfigDir) {
    console.log("no app config dir");
    await initAppDataPath();
    return;
  }
  const { root_path } = await readConfig();
  const projects: RustCallResult<DarftItemProps[]> = await invoke("projects", {
    appConfigPath: root_path,
  });
  projectsRef.value = projects.data;
}

onMounted(async () => {
  await loadAllProjects();
});

useInterval(3000, () => {
  loadAllProjects();
});

const filterProjects = computed(() => {
  if (!searchNameRef.value) {
    return projectsRef.value;
  } else {
    if (projectsRef.value) {
      return projectsRef.value.filter(
        (it) => it.draft_name.indexOf(searchNameRef.value) != -1
      );
    } else {
      return [];
    }
  }
});

/**
 * 模拟点击文件选择
 */
function fileSelect() {
  if (fileSelectRef.value) {
    fileSelectRef.value.click();
  }
}
</script>

<template>
  <div class="card">
    <input
      v-show="false"
      ref="fileSelectRef"
      type="file"
      webkitdirectory
      accept=".json"
      @change="onChange($event)"
      placeholder="请选择文件剪映草稿路径"
    />
    <div class="search">
      <n-space>
        <n-input
          clearable
          v-model:value="searchNameRef"
          style="width: 350px"
          size="large"
          round
          placeholder="搜索项目"
        />
        <n-button
          size="medium"
          title="选择剪映草稿目录下的root_meta_info.json"
          round
          @click="fileSelect()"
        >
          配置剪映草稿路径
        </n-button>
        <!-- <n-button
          size="medium"
          title="重新加载项目"
          round
          @click="loadAllProjects()"
          >重新加载项目
        </n-button> -->
      </n-space>
    </div>
    <div class="draft-div">
      <n-empty
        v-if="!filterProjects || !filterProjects.length"
        description="暂无匹配项目"
      >
      </n-empty>
      <n-grid :x-gap="15" :y-gap="15" item-responsive>
        <n-gi
          span="360:8 480:6 720:4 960:3 1440:2"
          v-for="project in filterProjects"
        >
          <DraftItemVue v-bind="project" />
        </n-gi>
      </n-grid>
    </div>
  </div>
</template>

<style scoped>
.search {
  width: 100%;
  display: flex;
  flex-direction: row;
  justify-content: center;
  margin: 0 15px 15px;
}
.draft-div {
  margin: 0 20px;
}
</style>
