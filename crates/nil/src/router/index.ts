import { gameRoutes } from './game';
import type { Route } from './types';
import { createRouter, createWebHistory } from 'vue-router';

export const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      component: () => import('@/scenes/home/index.vue'),
      name: 'home' satisfies Route,
      path: '/',
    },
    {
      component: () => import('@/scenes/host-game/index.vue'),
      name: 'host-game' satisfies Route,
      path: '/host-game',
    },
    {
      component: () => import('@/scenes/join-game/index.vue'),
      name: 'join-game' satisfies Route,
      path: '/join-game',
    },
    {
      component: () => import('@/scenes/settings/index.vue'),
      name: 'settings' satisfies Route,
      path: '/settings',
    },

    {
      children: gameRoutes,
      component: () => import('@/scenes/game/index.vue'),
      path: '/game',
    },
  ],
});

export function go(to: Route) {
  void router.push({ name: to });
}
