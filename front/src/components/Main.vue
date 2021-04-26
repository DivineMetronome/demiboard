<template>
  <div class="main-wrapper">
    <div class="welcome">Welcome to yet another toy imageboard</div>
    <div class="board-catalog">
      <router-link
        v-for="board in list"
        :key="board.code"
        class="board"
        :to="'/' + board.code"
      >
        <div>/{{ board.code }}/ - {{ board.name }}</div>
        <div class="board-description">{{ board.description }}</div>
      </router-link>
    </div>
  </div>
</template>

<script lang="ts" setup>
import useBoards from '../util/useBoards';

export const { list } = useBoards();
</script>

<style lang="scss" scoped>
.main-wrapper {
  margin: 0 20%;
  width: 60%;
  padding-top: 2em;
  & > * {
    background: #222;
    margin-bottom: 2em;
    padding: 1em;
  }
}

@media (max-width: 800px) {
  .main-wrapper {
    margin: 0 10%;
    padding-top: 1.5em;
    width: 80%;
  }
}
.welcome {
  text-align: center;
}
.board-catalog {
  display: flex;
  flex-wrap: wrap;
  justify-content: center;
}
.board {
  display: block;
  text-align: center;
  width: 170px;
  &-description {
    font-size: 0.8em;
    color: #cce;
  }
}
</style>
