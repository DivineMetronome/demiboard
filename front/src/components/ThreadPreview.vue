<template>
  <div class="thread-preview">
    <div>R:{{ thread.replies }} I:0</div>
    <div class="title">{{ thread.title }}</div>
    {{ thread.text }}
  </div>
</template>

<script lang="ts">
export default {
  props: ['thread'],
};
</script>

<style>
.thread-preview {
  width: 180px;
  max-height: 320px;
  overflow: hidden;
  margin: 1em;
  text-align: center;
}
.title {
  font-weight: bold;
}
</style>
