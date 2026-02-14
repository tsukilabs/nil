// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { gameRoutes } from './game';
import {
  createRouter,
  createWebHistory,
  type LocationQueryRaw,
  type RouteParams,
} from 'vue-router';

export const QUERY_JOIN_REMOTE_GAME_WORLD_ID = 'world';
export const QUERY_LOAD_LOCAL_GAME_PATH = 'path';
export const QUERY_SIGN_IN_USER = 'user';
export const QUERY_SIGN_UP_USER = 'user';

export const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      component: () => import('@/scenes/home/index.vue'),
      path: '/',
      name: 'home' satisfies Scene,
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
      component: () => import('@/scenes/host-remote-game/index.vue'),
      path: '/host-remote-game',
      name: 'host-remote-game' satisfies Scene,
    },
    {
      component: () => import('@/scenes/join-local-game/index.vue'),
      path: '/join-local-game',
      name: 'join-local-game' satisfies Scene,
    },
    {
      component: () => import('@/scenes/join-remote-game/index.vue'),
      path: '/join-remote-game',
      name: 'join-remote-game' satisfies Scene,
    },
    {
      component: () => import('@/scenes/load-local-game/index.vue'),
      path: '/load-local-game',
      name: 'load-local-game' satisfies Scene,
    },
    {
      component: () => import('@/scenes/lobby/index.vue'),
      path: '/lobby',
      name: 'lobby' satisfies Scene,
    },
    {
      component: () => import('@/scenes/settings/index.vue'),
      path: '/settings',
      name: 'settings' satisfies Scene,
    },
    {
      component: () => import('@/scenes/sign-in/index.vue'),
      path: '/sign-in',
      name: 'sign-in' satisfies Scene,
    },
    {
      component: () => import('@/scenes/sign-up/index.vue'),
      path: '/sign-up',
      name: 'sign-up' satisfies Scene,
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

export function isGameRoute() {
  return location.pathname.startsWith('/game');
}

export function previousRouteIsGame() {
  let state: unknown = router.options.history.state;
  state ??= history.state;

  if (state && typeof state === 'object') {
    const back = Reflect.get(state, 'back');
    return typeof back === 'string' && back.startsWith('/game');
  }
  else {
    return false;
  }
}

export async function goBackIfPreviousIsNotGame() {
  if (previousRouteIsGame()) {
    await go('home');
  }
  else {
    router.back();
  }
}
