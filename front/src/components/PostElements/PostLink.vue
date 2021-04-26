<template>
  <a
    ref="link"
    :class="{ 'link-opened': showPost }"
    @click.exact.prevent="togglePost"
    @mouseenter="showPostHover = true"
    @mouseleave="showPostHover = false"
    @click.shift="scrollToPost"
  >
    >>{{ post }}
  </a>
  <post
    v-if="showPost || showPostHover"
    ref="postPopup"
    :post="postInfo"
    :index="-1"
    :reply-map="replyMap"
    :posts="posts"
  />
</template>

<script lang="ts" setup="props">
import { Post as TPost } from '../../types';
import { ref, computed, Ref, watchEffect, ComponentPublicInstance } from 'vue';
import { ReplyMap } from '../../util/useUpdatedThread';
import { createPopper, Instance } from '@popperjs/core';

declare const props: {
  post: number;
  posts: TPost[];
  replyMap: ReplyMap;
};

const showPost = ref(false);
const showPostHover = ref(false);
const link: Ref<HTMLElement> = ref(null);
const popper: Ref<Instance> = ref(null);
const postPopup: Ref<ComponentPublicInstance> = ref(null);

const postInfo = computed(() => {
  const val = props.posts.find((p) => p.id === props.post);
  return val;
});

watchEffect(() => {
  if (showPostHover.value && !showPost.value && postPopup.value) {
    popper.value = createPopper(link.value, postPopup.value.$el, {
      placement: 'right',
      modifiers: [
        {
          name: 'offset',
          options: {
            offset: [0, 10],
          },
        },
      ],
    });
  }
});
watchEffect(() => {
  if ((!showPostHover.value || showPost.value) && popper.value) {
    popper.value.destroy();
  }
});

const scrollToPost = (): void => {
  location.hash = '#post-' + props.post;
};

const togglePost = (): void => {
  showPost.value = !showPost.value;
};

export {
  showPost,
  postInfo,
  togglePost,
  link,
  showPostHover,
  postPopup,
  scrollToPost,
};
</script>

<style scoped>
.link-opened {
  color: #ccf;
}
</style>
