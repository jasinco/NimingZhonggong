<script setup lang="ts">
const input = defineModel<string>();

defineProps<{
  prompt: string;
  passwd?: boolean;
}>();
</script>
<template>
  <div class="input-field w-40 h-10 lg:w-80 lg:h-15">
    <input
      type="text"
      class="input_box lg:text-xl text-sm"
      :class="passwd ? 'passwd' : ''"
      required
      v-model="input"
    />
    <span class="prompt">{{ prompt }}</span>
    <span class="indicator text-dark-red"><slot></slot></span>
  </div>
</template>
<style scoped>
.input-field {
  position: relative;
  margin: 0;
  padding: 0;
  box-sizing: border-box;
  outline: none;
  display: flex;
}
.input-field input {
  position: absolute;
  background: transparent;
  border: solid 2px;
  border-radius: 15px;
  box-shadow: none;
  font-size: 16px;
  color: #fff;
  width: 100%;
  height: 80%;
  top: 0;
  left: 0;
  z-index: 1;
}
.prompt {
  position: absolute;
  top: 50%;
  transform: translateY(-59%);
  left: 5%;
  z-index: 0;
}
.input_box:focus ~ .prompt,
.input_box:valid ~ .prompt {
  transition-duration: 0.5s;
  transform: translateY(-178%);
  left: 3px;
}
.input_box:focus,
.input_box:valid {
  transition-duration: 0.5s;
  border: none;
  border-bottom: solid;
  border-radius: 0;
  outline: none;
}
.input_box:not(:valid) ~ .indicator {
  display: none;
}
.indicator {
  position: absolute;
  right: 0;
  transform: translateX(120px);
}
.passwd {
  -webkit-text-security: circle;
  font-size: larger;
}
</style>
