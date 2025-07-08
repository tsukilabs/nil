// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { RouteRecordRaw } from 'vue-router';

export const gameRoutes: RouteRecordRaw[] = [
  {
    component: () => import('@/scenes/game/academy/index.vue'),
    name: 'academy' satisfies GameScene,
    path: 'academy',
  },
  {
    component: () => import('@/scenes/game/continent/index.vue'),
    name: 'continent' satisfies GameScene,
    path: 'continent',
  },
  {
    component: () => import('@/scenes/game/farm/index.vue'),
    name: 'farm' satisfies GameScene,
    path: 'farm',
  },
  {
    component: () => import('@/scenes/game/iron-mine/index.vue'),
    name: 'iron-mine' satisfies GameScene,
    path: 'iron-mine',
  },
  {
    component: () => import('@/scenes/game/prefecture/index.vue'),
    name: 'prefecture' satisfies GameScene,
    path: 'prefecture',
  },
  {
    component: () => import('@/scenes/game/quarry/index.vue'),
    name: 'quarry' satisfies GameScene,
    path: 'quarry',
  },
  {
    component: () => import('@/scenes/game/sawmill/index.vue'),
    name: 'sawmill' satisfies GameScene,
    path: 'sawmill',
  },
  {
    component: () => import('@/scenes/game/script/index.vue'),
    name: 'script' satisfies GameScene,
    path: 'script',
  },
  {
    component: () => import('@/scenes/game/silo/index.vue'),
    name: 'silo' satisfies GameScene,
    path: 'silo',
  },
  {
    component: () => import('@/scenes/game/stable/index.vue'),
    name: 'stable' satisfies GameScene,
    path: 'stable',
  },
  {
    component: () => import('@/scenes/game/village/index.vue'),
    name: 'village' satisfies GameScene,
    path: 'village',
  },
  {
    component: () => import('@/scenes/game/wall/index.vue'),
    name: 'wall' satisfies GameScene,
    path: 'wall',
  },
  {
    component: () => import('@/scenes/game/warehouse/index.vue'),
    name: 'warehouse' satisfies GameScene,
    path: 'warehouse',
  },
];
