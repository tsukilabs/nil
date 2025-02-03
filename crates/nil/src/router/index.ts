import { gameRoutes } from './game';
import type { Route } from './types';
import { createMemoryHistory, createRouter } from 'vue-router';

export const router = createRouter({
  history: createMemoryHistory(),
  routes: [
    {
      path: '/',
      name: 'home' satisfies Route,
      component: () => import('@/scenes/Home.vue'),
    },
    {
      path: '/host-game',
      name: 'host-game' satisfies Route,
      component: () => import('@/scenes/HostGame.vue'),
    },
    {
      path: '/join-game',
      name: 'join-game' satisfies Route,
      component: () => import('@/scenes/JoinGame.vue'),
    },
    {
      path: '/settings',
      name: 'settings' satisfies Route,
      component: () => import('@/scenes/Settings.vue'),
    },

    {
      path: '/game',
      component: () => import('@/scenes/Game.vue'),
      children: gameRoutes,
    },
  ],
});

export function go(to: Route) {
  void router.push({ name: to });
}
