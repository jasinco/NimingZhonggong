<script setup lang="ts">
import { ref } from "vue";
defineProps<{ prompt: string }>();
const pressed = defineModel<boolean>();
</script>
<template>
  <div
    class="w-full h-full border-solid rounded-xl border-white border-2 items-center grid z-3"
    :class="{ glow: pressed, 'bg-dark-bg_1': pressed }"
    @click.capture.click="(_) => (pressed = true)"
  >
    <p class="-translate-y-2 z-0">{{ prompt }}</p>
  </div>
</template>
<style scoped>
.glow {
  transition: 0.3s;
  text-shadow: #fc0 1px 0 10px;
}
</style>
