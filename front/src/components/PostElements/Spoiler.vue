<template>
  <span class="spoiler">
    <slot />
  </span>
</template>

<style scoped>
.spoiler {
  color: #000;
  background-color: #000;
}
.spoiler:hover {
  color: #ddd;
  background-color: #222;
}
</style>
