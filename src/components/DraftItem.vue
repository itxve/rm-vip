<script setup lang="ts">
export type DarftItemProps = {
  draft_cover: string;
  draft_json_file: string;
  draft_name: string;
  tm_duration: number;
  tm_draft_modified: number;
};
export type DarftProps = {
  all_draft_store: Array<DarftItemProps>;
  root_path: string;
};

import { readSysFileForArray, writeSysFileFromString } from "@/util";
import { ref, watchEffect, onUnmounted } from "vue";
import { NButton, useNotification } from "naive-ui";

const props = defineProps<DarftItemProps>();
const fileBlob = ref<string>();

const notification = useNotification();

watchEffect(async () => {
  await converntBlob(props.draft_cover);
});

async function converntBlob(cover: string) {
  if (!props.draft_cover) {
    return;
  }
  const { data, err } = await readSysFileForArray(cover);
  if (!err) {
    //ๅญๅจ็งป้ค
    if (fileBlob.value) {
      URL.revokeObjectURL(fileBlob.value);
    }
    fileBlob.value = URL.createObjectURL(
      new Blob([Uint8Array.from(data)], { type: "image/*" })
    );
  }
  return "";
}
onUnmounted(() => {
  if (fileBlob.value) {
    URL.revokeObjectURL(fileBlob.value);
  }
});

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
          content: "๐๐๐" + props.draft_name,
          meta: "็งปๅบVipๆๅ๐๐๐",
          duration: 2500,
        });
      }
    };
  }
}
</script>

<template>
  <img :src="fileBlob" alt="ๅพ็ๅ?่ฝฝๅคฑ่ดฅ" />
  <div>
    <span>
      {{ props.draft_name }}
    </span>
  </div>
  <div>
    <n-button type="error" size="small" @click="removeVip()">ๅป้คVip</n-button>
  </div>
</template>

<style scoped>
img {
  width: 120px;
  height: 80px;
  border-radius: 10px;
}
</style>
