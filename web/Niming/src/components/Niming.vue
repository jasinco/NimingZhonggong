<script setup lang="ts">
import axios from "axios";
import Niming_post from "./Niming_post.vue";
import { ref } from "vue";

let post_list = ref(new Array());
const update = () => {
  console.log("Updated");
  axios({ method: "get", url: "/niming/get" }).then((x) => {
    post_list.value.concat(x.data);
  });
};
</script>
<template>
  <div class="mvp niming_container bg-dark-bg" @scrollend="update">
    <h1 class="title">Niming</h1>
    <Niming_post
      title="reggin"
      content="Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Tortor condimentum lacinia quis vel eros. Metus aliquam eleifend mi in nulla. Orci eu lobortis elementum nibh. Magnis dis parturient montes nascetur ridiculus mus mauris. Commodo odio aenean sed adipiscing diam donec adipiscing tristique risus. A iaculis at erat pellentesque adipiscing commodo elit at. Id neque aliquam vestibulum morbi blandit cursus. Amet consectetur adipiscing elit pellentesque. Proin sagittis nisl rhoncus mattis. Velit ut tortor pretium viverra suspendisse.

Sed blandit libero volutpat sed cras ornare arcu dui vivamus. Aliquam id diam maemecenas ultricies mi eget mauris pharetra et. Justo nec ultrices dui sapien eget mi proin sed. Viverra orci sagittis eu volutpat odio facilisis mauris sit amet. Nulla porttitor massa id neque aliquam. Vel quam elementum pulvinar etiam. Non diam phasellus vestibulum lorem sed risus ultricies. Leo a diam sollicitudin tempor id. Justo eget magna fermentum iaculis. Quisque sagittis purus sit amet. Auctor eu augue ut lectus arcu bibendum. Varius quam quisque id diam vel quam elementum pulvinar.

Nunc lobortis mattis aliquam faucibus purus. Purus faucibus ornare suspendisse sed nisi lacus sed. Quam viverra orci sagittis eu volutpat odio facilisis mauris. Integer malesuada nunc vel risus commodo viverra. Integer eget aliquet nibh praesent tristique magna. Nam at lectus urna duis convallis convallis. Proin sagittis nisl rhoncus mattis. Facilisis volutpat est velit egestas dui. Porta nibh venenatis cras sed felis. Nunc sed id semper risus in hendrerit gravida rutrum. Ipsum suspendisse ultrices gravida dictum fusce. Porta non pulvinar neque laoreet suspendisse interdum consectetur. Risus sed vulputate odio ut. Nunc sed blandit libero volutpat sed cras ornare. Ac tortor dignissim convallis aenean et.

Sapien et ligula ullamcorper malesuada proin libero nunc consequat. Et egestas quis ipsum suspendisse ultrices gravida dictum fusce. Felis eget velit aliquet sagittis id. Eget aliquet nibh praesent tristique magna sit amet purus gravida. Eu sem integer vitae justo eget magna. Tristique senectus et netus et. Fames ac turpis egestas integer eget aliquet nibh. Cursus risus at ultrices mi tempus imperdiet nulla. Commodo odio aenean sed adipiscing diam donec adipiscing tristique. Sed risus pretium quam vulputate dignissim suspendisse in est ante. At lectus urna duis convallis. Egestas erat imperdiet sed euismod. Vitae congue mauris rhoncus aenean vel elit scelerisque. Mauris cursus mattis molestie a iaculis at. Nulla at volutpat diam ut venenatis tellus in metus. Pellentesque elit eget gravida cum sociis natoque penatibus et magnis. Congue nisi vitae suscipit tellus mauris a. Arcu non sodales neque sodales ut etiam sit amet.

Ornare arcu odio ut sem nulla pharetra diam sit. Mi tempus imperdiet nulla malesuada pellentesque elit eget gravida cum. Nunc sed blandit libero volutpat sed cras ornare arcu. Tellus orci ac auctor augue mauris augue neque gravida. Ullamcorper eget nulla facilisi etiam dignissim diam. Est ullamcorper eget nulla facilisi etiam dignissim diam quis enim. Consequat interdum varius sit amet. Senectus et netus et malesuada. Eu mi bibendum neque egestas. Eu lobortis elementum nibh tellus molestie nunc non blandit. Sed cras ornare arcu dui. In vitae turpis massa sed elementum tempus. Tortor dignissim convallis aenean et tortor. Sit amet luctus venenatis lectus. A pellentesque sit amet porttitor eget dolor morbi non arcu. Proin sagittis nisl rhoncus mattis rhoncus urna. Scelerisque purus semper eget duis. Ullamcorper morbi tincidunt ornare massa eget egestas purus viverra accumsan. Etiam dignissim diam quis enim."
    ></Niming_post>
  </div>
</template>
<style scoped>
.title {
  margin-top: 0;
}
.niming_container {
  padding: 0 10% 100dvh;
}
</style>
