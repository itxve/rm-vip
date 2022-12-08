import { onMounted, ref, onUnmounted } from "vue";
export default function (ms: number, fn: Function) {
  const interval = ref();
  onMounted(() => {
    interval.value = setInterval(fn, ms);
  });
  onUnmounted(() => {
    if (interval.value) {
      clearInterval(interval.value);
    }
  });
}
