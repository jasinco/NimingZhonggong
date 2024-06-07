<script setup lang="ts">
defineProps(["title", "content"]);
</script>
<template>
  <div class="w-60 h-60 border-solid border-zinc-500 rounded">
    <p class="text-3xl h-1/5 m-0">{{ title }}</p>
    <p class="text-ellipsis overflow-hidden h-4/5 m-0">{{ content }}</p>
  </div>
</template>
