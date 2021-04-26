<template>
  <div>
    <text-fragment
      v-for="token in tokens"
      :key="token.idx"
      :token="token"
      :posts="posts"
      :reply-map="replyMap"
    />
  </div>
</template>

<script lang="ts" setup="props">
import { PostToken } from '../../util/postTokenizer';
import { Post } from '../../types';
import { ReplyMap } from '../../util/useUpdatedThread';

declare const props: {
  tokens: PostToken[];
  posts: Post[];
  replyMap: ReplyMap;
};
</script>

<style></style>
