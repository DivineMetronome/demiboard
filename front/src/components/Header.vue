<template>
  <header>
    <div v-for="board in list" :key="board.code" class="board">
      <router-link :to="'/' + board.code" :title="board.name">{{
        board.code
      }}</router-link>
    </div>
  </header>
</template>
<script lang="ts" setup>
import useBoards from '../util/useBoards';

export const { list } = useBoards();
</script>
<style scoped>
header {
  height: 1rem;
  line-height: 1rem;
  background: #222;
  padding: 0.5rem;
  display: flex;
  justify-content: center;
}
.board:not(:last-child):after {
  content: '|';
  margin: 0 0.5rem;
}
</style>
