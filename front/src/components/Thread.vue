<template>
  <div class="thread">
    <post-form ref="postForm" @send-post="sendPost" />
    <div class="error">
      {{ error }}
    </div>
    <div v-if="thread" class="posts, { emit }">
      <thread-post
        v-for="(post, index) in thread.posts"
        :key="post.id"
        :post="post"
        :index="index"
        :reply-map="replyMap"
        :posts="thread.posts"
        @delete-post="deletePost"
        @reply-to="replyToPost"
      />
    </div>
  </div>
</template>

<script lang="ts" setup="props, { emit }">
import ThreadPost from './ThreadPost.vue';
import PostForm from './PostForm.vue';
import useUpdatedThread from '../util/useUpdatedThread';
import { ref } from 'vue';
import { NewPost, Emit } from '../types';

declare const emit: Emit;
declare const props: {
  board: string;
  threadId: string;
};

const postForm = ref(null);

const { thread, error, replyMap } = useUpdatedThread(props.threadId);

const deletePost = (id: number): void => {
  //posts.value.splice(index, 1);
};
const replyToPost = (postId: number): void => {
  postForm.value.open();
  postForm.value.addReplyLink(postId);
};

const sendPost = async (post: NewPost): Promise<void> => {
  try {
    const data = new FormData();
    const payload = { ...post };
    delete payload.file;
    delete payload.title;

    data.append('payload', JSON.stringify(payload));

    if (post.file) {
      data.append('file', post.file);
    }

    const response = await fetch('/api/thread/' + props.thread, {
      method: 'POST',
      body: data,
    });
    const result = await response.json();

    if (result.success) {
      //alert('success');
    } else {
      throw new Error(result.message);
    }
  } catch (err) {
    console.error(err);
    emit('toast', err);
  }
};

export { thread, error, replyMap, postForm, sendPost, deletePost, replyToPost };
export default {
  components: {
    'post-form': PostForm,
    'thread-post': ThreadPost,
  },
};
</script>
<style scoped lang="scss">
.thread {
  width: 100%;
  display: flex;
  flex-direction: column;
  justify-items: center;
}
.error {
  text-align: center;
  font-size: 2em;
  color: #f40;
}
</style>
