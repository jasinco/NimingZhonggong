<template>
  <div
    class="w-dvw grid lg:grid-cols-2 lg:grid-rows-1 vp sm:grid-rows-2 sm:grid-cols-1"
  >
    <RouterLink
      to="/vote"
      class="m-auto w-5/6 h-5/6 grid items-center blck text-dfg2 hover:bg-gruv-vlight2 hover:text-lfg1 text-dark-fg"
      >提案</RouterLink
    >
    <RouterLink
      to="/niming"
      class="m-auto w-5/6 h-5/6 grid items-center blck text-dfg2 text-dark-fg"
      >匿名版</RouterLink
    >
  </div>
</template>
<style>
.blck {
  font-size: 1.34rem;
  font-weight: 600;
  border-radius: 15px;
  border: solid 3px hsl(4, 0%, 78%);
  box-shadow: 10px 5px teal;
}
.blck:hover {
  box-shadow: 2px 3px teal;
}
</style>
