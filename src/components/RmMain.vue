<script setup lang="ts">
import { ref, onMounted, computed } from "vue";
import useListenEvent from "@/hooks/useListenEvent";
import { invoke } from "@tauri-apps/api/tauri";
import { JY_FILE_NAME, readConfig, writeConfig } from "@/util";
import { RustCallResult } from "@/types";

import DraftItemVue, {
  DarftProps,
  DarftItemProps,
} from "@/components/DraftItem.vue";
import { NButton, NGrid, NGi, NInput, useNotification } from "naive-ui";
const notification = useNotification();
const fileSelectRef = ref<HTMLInputElement>();
const projectsRef = ref<DarftItemProps[]>([]);
const searchNameRef = ref();

async function onChange(e: any) {
  let reader = new FileReader();
  if (!e.target.files && !e.target.files.length) {
    return;
  }

  const jyConfiigFile = [...e.target.files].find(
    (ft) => ft.name.indexOf(JY_FILE_NAME) != -1
  );

  if (!jyConfiigFile) {
    notification["error"]({
      content: "配置错误",
      meta: "不是一个正确的剪映文件",
      duration: 1500,
    });
    return;
  }
  reader.readAsText(e.target.files[0]);
  reader.onload = async function () {
    const result = reader.result as string;
    if (!result) {
      notification["error"]({
        content: "配置错误",
        meta: "内容为空",
        duration: 1500,
      });
      return;
    }
    const darftRef: DarftProps = JSON.parse(result);
    await writeConfig(JSON.stringify({ root_path: darftRef.root_path }));
    // 保存后重新加载
    await loadAllProjects();
  };
}

async function loadAllProjects() {
  const { root_path } = await readConfig();
  const projects: RustCallResult<DarftItemProps[]> = await invoke("projects", {
    appConfigPath: root_path,
  });
  projectsRef.value = projects.data;
}

/**
 * 事件监听
 */
useListenEvent<number>("jy_file_change", (e) => {
  loadAllProjects();
});

onMounted(async () => {
  await loadAllProjects();
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
      accept=".json"
      @change="onChange($event)"
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
          :key="project.tm_draft_modified"
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
