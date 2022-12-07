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
import { ref, watchPostEffect, onMounted } from "vue";
import { NButton } from "naive-ui";

const props = defineProps<DarftItemProps>();
const fileBlob = ref<string>();

onMounted(() => {
  watchPostEffect(async () => {
    if (props.draft_cover) {
      await converntBlob(props.draft_cover);
    }
  });
});

async function converntBlob(cover: string) {
  const { data, err } = await readSysFileForArray(cover);
  if (!err) {
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
      console.log("fRead.result", fRead.result);
      let draft = fRead.result as string;
      // let flagVip = true;
      draft = JSON.parse(draft, (key, value) => {
        if (["resource_id", "formula_id"].includes(key)) {
          // flagVip = !!value;
          return "";
        } else {
          return value;
        }
      });

      let writeRt = await writeSysFileFromString(
        props.draft_json_file,
        JSON.stringify(draft)
      );
      if (writeRt) {
        alert("移除成功");
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
  <div><n-button type="error" @click="removeVip()">去除Vip</n-button></div>
</template>

<style scoped>
img {
  width: 120px;
  height: 80px;
  border-radius: 10px;
}
</style>
