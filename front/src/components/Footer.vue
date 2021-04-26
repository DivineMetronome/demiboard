<template>
  <footer>
    Demiboard, {{ currentYear }} 🄯 Sotha Sil, all rights relinquished
  </footer>
</template>
<script lang="ts" setup>
export const currentYear = new Date().getFullYear();
</script>
<style scoped>
footer {
  font-size: 0.8rem;
  line-height: 0.8rem;
  height: 0.8rem;
  background: #222;
  padding: 0.4rem;
  text-align: center;
}
</style>
