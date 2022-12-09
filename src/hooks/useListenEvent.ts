import { onMounted, ref, onUnmounted } from "vue";
import { listen, EventCallback, UnlistenFn } from "@tauri-apps/api/event";

export default function <T>(eventName: string, fn: EventCallback<T>) {
  const listenRef = ref<Promise<UnlistenFn>>();
  onMounted(() => {
    listenRef.value = listen<T>(eventName, fn);
  });
  onUnmounted(async () => {
    if (listenRef.value) {
      const unListen = await listenRef.value;
      unListen();
    }
  });
}
