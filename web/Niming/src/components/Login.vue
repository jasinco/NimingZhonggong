<script setup lang="ts">
import { ref, watch } from "vue";
import Idendtify_input from "./Idendtify_input.vue";
const id = defineModel<string>("id");
const password = defineModel<string>("password");
const id_indicator = ref("");
const RegexID = RegExp(/^[0-9]{6}$/g);

watch(id, async (next) => {
  if (next?.match(RegexID) == null) {
    id_indicator.value = "非正確學號格式";
  } else {
    id_indicator.value = "";
  }
});
</script>
<template>
  <div class="vp flex place-items-center flex-wrap flex-col gap-6">
    <h1 class="h-fit mb-0">登入</h1>
    <div class="w-2/3 h-7/10 flex flex-col flex-wrap place-items-center gap-7">
      <Idendtify_input prompt="學號" class="ml-a mr-a" v-model="id">{{
        id_indicator
      }}</Idendtify_input>
      <Idendtify_input
        prompt="密碼"
        class="ml-a mr-a"
        v-model="password"
      ></Idendtify_input>
      <button class="w-18 h-7 ml-a mr-a">確認</button>
    </div>
  </div>
</template>
