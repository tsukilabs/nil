// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { scriptRoutes } from './script';
import { profileRoutes } from './profile';
import type { RouteRecordRaw } from 'vue-router';
import { infrastructureRoutes } from './infrastructure';

export const gameRoutes: RouteRecordRaw[] = [
  {
    component: () => import('@/scenes/game/chat/index.vue'),
    path: 'chat',
    name: 'chat' satisfies GameScene,
  },
  {
    component: () => import('@/scenes/game/continent/index.vue'),
    path: 'continent',
    name: 'continent' satisfies GameScene,
  },
  {
    component: () => import('@/scenes/game/infrastructure/index.vue'),
    path: 'infrastructure',
    children: infrastructureRoutes,
  },
  {
    component: () => import('@/scenes/game/profile/index.vue'),
    path: 'profile',
    children: profileRoutes,
  },
  {
    component: () => import('@/scenes/game/ranking/index.vue'),
    path: 'ranking',
    name: 'ranking' satisfies GameScene,
  },
  {
    component: () => import('@/scenes/game/script/index.vue'),
    path: 'script',
    children: scriptRoutes,
  },
  {
    component: () => import('@/scenes/game/city/index.vue'),
    path: 'city',
    name: 'city' satisfies GameScene,
  },
];
