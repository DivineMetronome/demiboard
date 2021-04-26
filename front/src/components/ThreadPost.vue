<template>
  <div
    :id="'post-' + post.id"
    class="post-wrapper"
    :class="{ 'post-hidden': hidden }"
  >
    <div class="post-buttons">
      <span class="post-delete" title="Delete" @click="deletThis">
        <font-awesome-icon icon="trash" fixed-width />
      </span>
      <span class="post-hide" title="Hide" @click="hide">
        <font-awesome-icon icon="eye-slash" fixed-width />
        <font-awesome-icon icon="eye" fixed-width />
      </span>
    </div>
    <post :post="post" :index="index" :reply-map="replyMap" :posts="posts" />
  </div>
</template>

<script lang="ts" setup="props, { emit }">
import { Post, Emit } from '../types';
import { ref } from 'vue';
import { ReplyMap } from '../util/useUpdatedThread';

declare const emit: Emit;
declare const props: {
  post: Post;
  replyMap: ReplyMap;
  posts: Post[];
  index: number;
};
const hidden = ref(false);
const hide = (): void => {
  hidden.value = !hidden.value;
};

export { hidden, hide };
</script>

<style lang="scss">
.post-wrapper {
  display: flex;
  margin-bottom: 1em;
}

.post-buttons {
  display: flex;
  flex-direction: column;
  padding-right: 0.5em;
  align-items: center;
  justify-content: center;
}
.post-delete {
  color: #f40;
}
.post-delete:hover {
  color: #f86;
}
.post-hide:hover {
  color: #fff;
}
.post-hidden .delete,
.post-hidden .reply,
.post-hidden .post-body,
.post-hidden .fa-eye-slash,
.post-hidden .fa-tash,
.post-hidden .fa-pen {
  display: none;
}

.post-hidden .post-head {
  border: none;
}

.post-wrapper .fa-eye {
  display: none;
}
.post-hidden .fa-eye {
  display: inline-block;
}
.post-hidden .post-head {
  margin: 0;
}
</style>
