<script setup lang="ts">
import axios from "axios";
import { ref } from "vue";

const input_event = (e: Event) => {
  const target = e.target;
  if (target instanceof HTMLInputElement) {
    const val = target.value;

    if (val != "" && val !== null) {
      const next = target.nextElementSibling;
      if (next instanceof HTMLInputElement) {
        next.focus();
      }
    }
  }
};
const keyup_event = (e: KeyboardEvent) => {
  if (e.target instanceof HTMLInputElement) {
    const target = e.target;
    const key = e.key.toLowerCase();

    if (key == "backspace" || key == "delete") {
      target.value = "";
      const prev = target.previousElementSibling;
      if (prev instanceof HTMLInputElement) {
        prev.focus();
      }
      return;
    }
    if (key == "arrowright") {
      const next = target.nextElementSibling;
      if (next instanceof HTMLInputElement) {
        next.focus();
      }
    } else if (key == "arrowleft") {
      const prev = target.previousElementSibling;
      if (prev instanceof HTMLInputElement) {
        prev.focus();
      }
    }
  }
};

const gen_code = () => {
  axios({ method: "get", url: "/verify/gen" })
    .then((resp) => {
      console.log(resp);
    })
    .catch((resp) => {
      console.log(resp);
    });
};

let codes = ref<string[]>(new Array(8).fill(""));

const validate = () => {
  let return_string = codes.value.join("");
  console.log(return_string);
};

gen_code();
</script>
<template>
  <div class="flex flex-wrap flex-col gap-5 w-fit m-a items-center">
    <h1>Verify OTP CODE</h1>
    <div class="code_block" @input="input_event" @keyup="keyup_event">
      <input
        type="text"
        required
        maxlength="1"
        class="code"
        v-for="(_, id) in codes"
        v-model="codes[id]"
      />
    </div>
    <button
      @click="validate"
      class="border-dark-blue border-solid rounded w-18 h-7"
    >
      確認
    </button>
  </div>
</template>
<style scoped>
.code {
  width: 38px;
  height: 38px;
  text-align: center;
  margin: 0 5px;
}
</style>
