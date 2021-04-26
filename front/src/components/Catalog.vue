<template>
  <div class="catalog">
    <div class="catalog-head">
      <button @click="openPostForm">Make a thread</button>
    </div>

    <div class="catalog-content">
      <div v-if="threads.error">
        {{ error }}
      </div>
      <div v-else>
        <thread-preview
          v-for="thread in threads.value"
          :key="thread.id"
          :thread="thread"
        />
      </div>
      <post-form ref="postForm" :is-thread="true" @send-post="sendThread" />
    </div>
  </div>
</template>
<script lang="ts" setup="props, { emit }">
import ThreadPreview from './ThreadPreview.vue';
import PostForm from './PostForm.vue';
import onMountedFetch from '../util/onMountedFetch';
import { ref } from 'vue';
import { NewThread, Emit } from '../types';
import { useRouter } from 'vue-router';

declare const props: {
  board: string;
};
declare const emit: Emit;

const threads = onMountedFetch('/api/boards/' + props.board + '/catalog', []);
const postForm = ref(null);
const router = useRouter();

const openPostForm = (): void => postForm.value.open();

const sendThread = async (post: NewThread): Promise<void> => {
  try {
    if (!post.file) {
      throw new Error('Image is required');
    }

    const data = new FormData();
    const payload = { ...post };
    delete payload.file;
    data.append('payload', JSON.stringify(payload));
    data.append('file', post.file);

    const response = await fetch('/api/boards/' + props.board, {
      method: 'POST',
      body: data,
    });
    const result = await response.json();

    if (result.success) {
      router.push(`${props.board}/${result.thread.id}`);
    } else {
      throw new Error(result.message);
    }
  } catch (err) {
    console.error(err);
    emit('toast', err);
  }
};

export { threads, postForm, openPostForm, sendThread };

export default {
  components: {
    'thread-preview': ThreadPreview,
    'post-form': PostForm,
  },
};
</script>
<style>
.catalog {
  display: flex;
  flex-direction: column;
  align-items: center;
  width: 100%;
}
.catalog-content {
  display: flex;
  flex-wrap: wrap;
  justify-content: center;
}
</style>
