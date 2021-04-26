import { RouteRecordRaw, createRouter, createWebHistory } from 'vue-router';
import Catalog from '../components/Catalog.vue';
import Thread from '../components/Thread.vue';
import Main from '../components/Main.vue';

const routes: RouteRecordRaw[] = [
  {
    path: '/:board',
    name: 'Board catalog',
    component: Catalog,
    props: true,
  },
  {
    path: '/:board/:threadId/',
    name: 'Thread',
    component: Thread,
    props: true,
  },
  {
    path: '/',
    name: 'Main page',
    component: Main,
  },
];

const router = createRouter({
  history: createWebHistory(process.env.BASE_URL),
  routes,
});

export default router;
