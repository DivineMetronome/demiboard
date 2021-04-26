import { reactive, Ref, markRaw, toRefs } from 'vue';
import { Thread, Post } from '../types';
import tokenize from './postTokenizer';

export interface ReplyMap {
  [reply: number]: number[];
}

interface SseThread {
  thread?: Ref<Thread>;
  replyMap: Ref<ReplyMap>;
  error?: Ref<string>;
}

interface SseEvent extends Event {
  data: string;
}

export default function useUpdatedThread(threadId: string): SseThread {
  const state = reactive({
    thread: null,
    replyMap: {},
    initialized: false,
    error: null,
  });
  const sse = new EventSource('/api/sse/thread/' + threadId);

  sse.onopen = () => {
    state.initialized = true;
  };

  sse.addEventListener('thread', (event: SseEvent) => {
    const payload: Thread = JSON.parse(event.data);
    if (!payload) {
      state.error = 'Thread not found';
      sse.close();
      return;
    }

    if (!payload.open) {
      state.error = 'This thread is closed';
      sse.close();
    }
    for (const post of payload.posts) {
      const { tokens, links } = tokenize(post.message);
      // making a huge tree reactive is damn slow
      // pinky swear the text won't change
      post.messageTokens = markRaw(tokens);
      post.links = links;
      state.replyMap[post.id] = [];
    }

    for (const post of payload.posts) {
      for (const link of post.links) {
        state.replyMap[link] && state.replyMap[link].push(post.id);
      }
    }

    state.thread = payload;
  });

  sse.addEventListener('post', (event: SseEvent) => {
    const payload: Post = JSON.parse(event.data);

    const { tokens, links } = tokenize(payload.message);
    payload.messageTokens = markRaw(tokens);
    payload.links = links;

    for (const link of payload.links) {
      state.replyMap[link] && state.replyMap[link].push(payload.id);
    }

    state.thread.posts.push(payload);
  });

  sse.onerror = () => {
    state.error = 'Something happened, tough titties';
    // don't bother reconnecting if it couldn't connect in the first place
    if (!state.initialized) {
      sse.close();
    }
  };

  return toRefs(state);
}
