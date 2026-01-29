// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { gameRoutes } from './game';
import { onlineRoutes } from './online';
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
      component: () => import('@/scenes/host-local-game/index.vue'),
      path: '/host-local-game',
      name: 'host-local-game' satisfies Scene,
    },
    {
      component: () => import('@/scenes/join-local-game/index.vue'),
      path: '/join-local-game',
      name: 'join-local-game' satisfies Scene,
    },
    {
      component: () => import('@/scenes/load-local-game/index.vue'),
      path: '/load-local-game',
      name: 'load-local-game' satisfies Scene,
    },
    {
      component: () => import('@/scenes/online/index.vue'),
      path: '/online',
      children: onlineRoutes,
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
