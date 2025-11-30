// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { gameRoutes } from './game';
import {
  createRouter,
  createWebHistory,
  type LocationQueryRaw,
  type RouteParams,
} from 'vue-router';

export const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      component: () => import('@/scenes/home/index.vue'),
      name: 'home' satisfies Scene,
      path: '/',
    },
    {
      component: () => import('@/scenes/about/index.vue'),
      path: '/about',
      name: 'about' satisfies Scene,
    },
    {
      component: () => import('@/scenes/game/index.vue'),
      path: '/game',
      children: gameRoutes,
    },
    {
      component: () => import('@/scenes/host-game/index.vue'),
      path: '/host-game',
      name: 'host-game' satisfies Scene,
    },
    {
      component: () => import('@/scenes/join-game/index.vue'),
      path: '/join-game',
      name: 'join-game' satisfies Scene,
    },
    {
      component: () => import('@/scenes/load-game/index.vue'),
      path: '/load-game',
      name: 'load-game' satisfies Scene,
    },
    {
      component: () => import('@/scenes/settings/index.vue'),
      path: '/settings',
      name: 'settings' satisfies Scene,
    },
  ],
});

export interface RouteOptions {
  params?: Option<RouteParams>;
  query?: Option<LocationQueryRaw>;
}

export function go(scene: Scene, options?: RouteOptions) {
  return router.push({
    name: scene,
    params: options?.params ?? undefined,
    query: options?.query ?? undefined,
  });
}
