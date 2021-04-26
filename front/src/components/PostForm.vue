<template>
  <div
    v-show="show"
    ref="el"
    class="post-form"
    :style="{ top: top, left: left }"
  >
    <div class="post-form-header">
      <div class="post-form-header__title" @mousedown.prevent="dragStart">
        {{ title }}
      </div>
      <font-awesome-icon icon="window-close" @click="close" />
    </div>
    <div class="post-form-content">
      <input
        v-model="post.name"
        type="text"
        placeholder="Anonymous"
        maxlength="50"
      />
      <input
        v-if="isThread"
        v-model="post.title"
        type="text"
        placeholder="Thread title"
        maxlength="100"
      />
      <textarea
        v-model="post.message"
        cols="30"
        rows="10"
        placeholder="Your message"
        maxlength="5000"
      />
      <input type="file" @change="onFileChange" />
      <button type="submit" @click="sendPost">Submit</button>
    </div>
  </div>
</template>

<script lang="ts" setup="props, { emit }">
import { library } from '@fortawesome/fontawesome-svg-core';
import { faWindowClose } from '@fortawesome/free-solid-svg-icons';
import { computed, ref, reactive } from 'vue';
import { Emit, NewThread } from '../types';

declare const props: {
  isThread: boolean;
};
declare const emit: Emit;

library.add(faWindowClose);

const show = ref(false);
const position = reactive({
  currentX: document.documentElement.clientWidth / 1.5,
  currentY: document.documentElement.clientHeight / 2,
  offsetX: 0,
  offsetY: 0,
});
const post = reactive({
  name: '',
  message: '',
  title: '',
  file: null,
}) as NewThread;
const el = ref(null);

const top = computed(() => {
  let top =
    ((position.currentY + position.offsetY) /
      document.documentElement.clientHeight) *
    100;

  if (top > 95) top = 95;
  else if (top < 0) top = 0;

  return top + '%';
});

const left = computed(() => {
  let left =
    ((position.currentX + position.offsetX) /
      document.documentElement.clientWidth) *
    100;

  if (left > 95) left = 95;
  else if (left < 0) left = 0;

  return left + '%';
});

const title = computed(() =>
  props.isThread ? 'New thread' : 'Reply to thread',
);

const dragElement = (event) => {
  event.preventDefault();
  position.currentX = event.clientX;
  position.currentY = event.clientY;
};

const dragStop = () => {
  document.removeEventListener('mouseup', dragStop);
  document.removeEventListener('mousemove', dragElement);
};

const dragStart = (event: MouseEvent): void => {
  position.currentX = event.clientX;
  position.currentY = event.clientY;
  position.offsetX = el.value.offsetLeft - event.clientX;
  position.offsetY = el.value.offsetTop - event.clientY;

  document.addEventListener('mousemove', dragElement);
  document.addEventListener('mouseup', dragStop);
};

const close = (): void => {
  show.value = false;
};

const open = (): void => {
  show.value = true;
};

const addReplyLink = (replyId: number): void => {
  post.message = '>>' + replyId + '\n' + post.message;
};

const sendPost = (): void => {
  emit('send-post', post);
};

const onFileChange = ({ target }: { target: HTMLInputElement }): void => {
  post.file = target.files[0];
};

export {
  title,
  show,
  el,
  top,
  left,
  post,
  dragStart,
  close,
  open,
  onFileChange,
  sendPost,
  addReplyLink,
};
</script>

<style scoped lang="scss">
.post-form {
  background: #333;
  border: 2px solid #aaa;
  padding: 0.5em;
  position: fixed;
  display: flex;
  box-sizing: box;
  flex-direction: column;
  justify-content: center;
  &-content {
    display: inherit;
    flex-direction: inherit;
    & > * {
      background: #efefef;
      color: #111;
      border: 1px solid #999;
      margin-bottom: 0.5em;
      padding: 0.5em;
    }
  }
  &-header {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 0.5em;
    &__title {
      text-align: left;
      width: 100%;
      cursor: move;
    }
  }
}
.fa-window-close:hover {
  color: #fff;
}
</style>
