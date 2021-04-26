<template>
  <boards-header />
  <main>
    <router-view />
  </main>
  <d-footer />
</template>

<script lang="ts">
import Header from './components/Header.vue';
import Footer from './components/Footer.vue';

export default {
  components: {
    'boards-header': Header,
    'd-footer': Footer,
  },
};
</script>

<style lang="scss">
* {
  padding: 0;
  margin: 0;
  font-family: Roboto, sans-serif;
}
body {
  background: #444;
  color: #ddd;
}
#demiboard {
  display: flex;
  flex-direction: column;
  min-height: 100vh;
}
main {
  flex-grow: 1;
  display: flex;
  align-items: stretch;
  padding: 1em;
}
a {
  color: #99e;
  text-decoration: none;

  &:hover {
    color: #ccf;
  }
}
</style>
