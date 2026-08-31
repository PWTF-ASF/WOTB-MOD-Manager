import { createMemoryHistory, createRouter, createWebHistory, type RouteRecordRaw } from 'vue-router'
import HomePage from '@/views/HomePage.vue'

export const routes: RouteRecordRaw[] = [
  {
    path: '/',
    component: HomePage,
    children: [
      { path: '', redirect: '/library' },
      {
        path: 'library',
        name: 'library',
        component: () => import('@/components/ModLibrary.vue'),
        meta: { keepAlive: true },
      },
      {
        path: 'settings',
        name: 'settings',
        component: () => import('@/components/Settings.vue'),
        meta: { keepAlive: true },
      },
    ],
  },
  { path: '/:pathMatch(.*)*', redirect: '/library' },
]

const router = createRouter({
  history: typeof window === 'undefined' ? createMemoryHistory() : createWebHistory(),
  routes,
})

export default router
