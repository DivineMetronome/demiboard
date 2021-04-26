import { createApp } from 'vue';
import App from './App.vue';
import router from './router';
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome';

import { library } from '@fortawesome/fontawesome-svg-core';
import {
  faMinus,
  faPen,
  faTrash,
  faEyeSlash,
  faEye,
} from '@fortawesome/free-solid-svg-icons';
library.add(faMinus, faPen, faTrash, faEyeSlash, faEye);

// it's recursive, so we have to register it globally
import TextFragment from './components/PostElements/TextFragment.tsx';
import Post from './components/PostElements/Post.vue';

createApp(App)
  .use(router)
  .component('text-fragment', TextFragment)
  .component('post', Post)
  .component('font-awesome-icon', FontAwesomeIcon)
  .mount('#demiboard');
