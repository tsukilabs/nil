// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { gameRoutes } from './game';
import { createRouter, createWebHistory } from 'vue-router';

export const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      component: () => import('@/scenes/home/index.vue'),
      name: 'home' satisfies Scene,
      path: '/',
    },
    {
      component: () => import('@/scenes/host-game/index.vue'),
      name: 'host-game' satisfies Scene,
      path: '/host-game',
    },
    {
      component: () => import('@/scenes/join-game/index.vue'),
      name: 'join-game' satisfies Scene,
      path: '/join-game',
    },
    {
      component: () => import('@/scenes/lobby/index.vue'),
      name: 'lobby' satisfies Scene,
      path: '/lobby',
    },
    {
      component: () => import('@/scenes/settings/index.vue'),
      name: 'settings' satisfies Scene,
      path: '/settings',
    },

    {
      component: () => import('@/scenes/game/index.vue'),
      path: '/game',
      children: gameRoutes,
    },
  ],
});

export function go(to: Scene) {
  return router.push({ name: to });
}
