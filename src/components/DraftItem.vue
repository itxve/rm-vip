<script setup lang="ts">
export type DarftItemProps = {
  draft_cover: string;
  draft_json_file: string;
  draft_name: string;
  tm_duration: number;
};
export type DarftProps = {
  all_draft_store: Array<DarftItemProps>;
  root_path: string;
};

import { readSysFileForArray, writeSysFileFromString } from "@/util";
import { ref, watchEffect, onMounted } from "vue";
import { NButton, useNotification } from "naive-ui";

import useInterval from "@/hooks/useInterval";

const props = defineProps<DarftItemProps>();
const fileBlob = ref<string>();

const notification = useNotification();

watchEffect(async () => {
  await converntBlob(props.draft_cover);
});

useInterval(8000, () => {
  console.log("draft_cover:::");
  converntBlob(props.draft_cover);
});

async function converntBlob(cover: string) {
  if (!props.draft_cover) {
    return;
  }
  const { data, err } = await readSysFileForArray(cover);
  if (!err) {
    //存在移除
    if (fileBlob.value) {
      URL.revokeObjectURL(fileBlob.value);
    }
    fileBlob.value = URL.createObjectURL(
      new Blob([Uint8Array.from(data)], { type: "image/*" })
    );
  }
  return "";
}

async function removeVip() {
  if (!props.draft_json_file) {
    return;
  }
  const { data, err } = await readSysFileForArray(props.draft_json_file);
  if (!err) {
    const fRead = new FileReader();
    fRead.readAsText(
      new Blob([Uint8Array.from(data)], { type: "application/json" })
    );
    fRead.onloadend = async function () {
      let draft = fRead.result as string;
      // let flagVip = true;
      draft = JSON.parse(draft, (key, value) => {
        if (["resource_id", "formula_id"].includes(key)) {
          // flagVip = !!value;
          return "_vip";
        } else {
          return value;
        }
      });

      let writeRt = await writeSysFileFromString(
        props.draft_json_file,
        JSON.stringify(draft)
      );
      if (writeRt) {
        notification["success"]({
          content: "🚕🚕🚕" + props.draft_name,
          meta: "移出Vip成功👌👌👌",
          duration: 2500,
        });
      }
    };
  }
}
</script>

<template>
  <img :src="fileBlob" alt="图片加载失败" />
  <div>
    <span>
      {{ props.draft_name }}
    </span>
  </div>
  <div>
    <n-button type="error" size="small" @click="removeVip()">去除Vip</n-button>
  </div>
</template>

<style scoped>
img {
  width: 120px;
  height: 80px;
  border-radius: 10px;
}
</style>
