<script setup lang="ts">
// import HelloWorld from "./components/HelloWorld.vue";
</script>

<template>
  <div
    class="flex flex-nowrap place-items-end gap-3.5 items-center p-a text-dark-fg"
    id="nav"
  >
    <RouterLink to="/" class="ml-8 text-3xl m-a text-dark-fg"
      >中工匿名</RouterLink
    >
    <RouterLink to="/login" class="ml-auto text-dark-fg">登入</RouterLink>
    <RouterLink to="/register" class="text-dark-fg">註冊</RouterLink>
    <RouterLink to="/license" class="mr-5 text-dark-fg">服務條款</RouterLink>
  </div>
  <RouterView></RouterView>
</template>

<style scoped>
.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: filter 300ms;
}
.logo:hover {
  filter: drop-shadow(0 0 2em #646cffaa);
}
.logo.vue:hover {
  filter: drop-shadow(0 0 2em #42b883aa);
}
#nav {
  height: 15%;
}
</style>
