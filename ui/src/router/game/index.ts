// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { RouteRecordRaw } from 'vue-router';
import { infrastructureRoutes } from './infrastructure';

export const gameRoutes: RouteRecordRaw[] = [
  {
    component: () => import('@/scenes/game/continent/index.vue'),
    name: 'continent' satisfies GameScene,
    path: 'continent',
  },
  {
    component: () => import('@/scenes/game/script/index.vue'),
    name: 'script' satisfies GameScene,
    path: 'script',
  },
  {
    component: () => import('@/scenes/game/village/index.vue'),
    name: 'village' satisfies GameScene,
    path: 'village',
  },

  {
    component: () => import('@/scenes/game/infrastructure/index.vue'),
    path: 'infrastructure',
    children: infrastructureRoutes,
  },
];
