<template>
  <div class="post">
    <div class="post-head">
      <span class="post-user">{{ user }}</span>
      <span class="post-date">{{ date }}</span>
      <span v-if="index >= 0" class="post-index">#{{ index + 1 }}</span>
      <a title="reply" class="post-id" @click="reply">>>{{ post.id }}</a>
    </div>
    <post-text
      class="post-body"
      :tokens="post.messageTokens"
      :posts="posts"
      :reply-map="replyMap"
    />
    <div v-if="replies.length" class="post-replies">
      <post-link
        v-for="link in replies"
        :key="link"
        :post="link"
        :posts="posts"
        :reply-map="replyMap"
      />
    </div>
  </div>
</template>

<script lang="ts" setup="props, { emit }">
import PostText from './PostText.vue';
import PostLink from './PostLink.vue';
import { Post, Emit } from '../types';
import { ref, computed } from 'vue';
import { ReplyMap } from '../util/useUpdatedThread';

declare const emit: Emit;
declare const props: {
  post: Post;
  replyMap: ReplyMap;
  posts: Post[];
  index: number;
};
const hidden = ref(false);
const date = computed(() =>
  new Date(props.post.timestamp * 1000).toLocaleString(),
);
const user = computed(() => props.post.name || 'Anonymous');
const replies = computed(() => props.replyMap[props.post.id]);

const hide = (): void => {
  hidden.value = !hidden.value;
};
const reply = (): void => emit('reply-to', props.post.id);
const deletThis = (): void => emit('delete-post', props.index);

export { replies, hidden, date, user, hide, reply, deletThis };

export default {
  components: {
    'post-text': PostText,
    'post-link': PostLink,
  },
};
</script>

<style lang="scss">
.post-wrapper {
  display: flex;
  margin-bottom: 1em;
}
.post {
  border: 2px solid #aaa;
  background: #333;

  &-head {
    padding: 0.2em 0.5em;
  }
  &-body {
    padding: 0.5em;
  }
  &-head {
    border-bottom: 2px solid #aaa;
  }
  &-head > *:not(:last-child) {
    margin-right: 0.5em;
  }
  &-user {
    color: #2a2;
  }
  &-index {
    color: #2a2;
  }
  &-id {
    color: #99e;
    cursor: pointer;
  }
  &-id:hover {
    color: #ccf;
  }
  &-replies {
    border-top: 2px solid #aaa;
    padding: 0.2em 0.5em;
  }
}

.buttons {
  display: flex;
  flex-direction: column;
  padding-right: 0.5em;
  align-items: center;
  justify-content: center;
}
.delete {
  color: #f40;
}
.delete:hover {
  color: #f86;
}
.edit {
  color: #77d;
}
.edit:hover {
  color: #aaf;
}
.hide:hover {
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
.fa-eye {
  display: none;
}
.post-hidden .fa-eye {
  display: inline-block;
}
.post-hidden .post-head {
  margin: 0;
}</style
>user
