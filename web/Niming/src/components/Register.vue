<script setup lang="ts">
import { watch } from "vue";
import Idendtify_input from "./Idendtify_input.vue";
import { useRouter } from "vue-router";
import Option_box from "./Option_box.vue";
import { VerifyType, useUserStore } from "../store";

const nameref = defineModel<string>("name");
const password = defineModel<string>("password");

const router = useRouter();

let p1 = defineModel<boolean>("pressed1");
let p2 = defineModel<boolean>("pressed2");
watch([p1, p2], (x, y) => {
  if (y[0] && x[0] == x[1] && x[0] == true) {
    p1.value = false;
  }
  if (y[1] && x[0] == x[1] && x[0] == true) {
    p2.value = false;
  }
});

const store = () => {
  if (nameref.value !== undefined && password.value !== undefined) {
    const store = useUserStore();
    store.assign_name(nameref.value);
    store.assign_password(password.value);
    store.assign_verify_type(p1.value ? VerifyType.email : VerifyType.totp);
    console.log(store.debug);
    router.push({ path: "/verify" });
  }
};
</script>

<template>
  <div class="vp flex items-center flex-col flex-wrap">
    <h1>註冊</h1>
    <form
      class="w-3/5 flex items-center flex-col flex-wrap gap-6"
      @submit.prevent
    >
      <Idendtify_input prompt="名字" v-model="nameref" />
      <Idendtify_input prompt="密碼" :passwd="true" v-model="password" />
      <div class="w-40 h-10 lg:w-80 lg:h-15 grid grid-cols-2 gap-5">
        <Option_box prompt="email" v-model="p1" />
        <Option_box prompt="TOTP 2FA" v-model="p2" />
      </div>
      <button
        type="submit"
        class="border-dark-blue border-solid rounded w-18 h-7"
        @click="store"
      >
        確認
      </button>
    </form>
  </div>
</template>
