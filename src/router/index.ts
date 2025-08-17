import { createRouter, createWebHistory } from 'vue-router';
import HomePage from '@/views/HomePage.vue';
import SettingsPage from '@/views/SettingsPage.vue';

const routes = [
  { path: '/', component: HomePage },
  { path: '/settings', component: SettingsPage },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
