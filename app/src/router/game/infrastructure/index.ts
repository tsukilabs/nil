// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { stableRoutes } from './stable';
import { academyRoutes } from './academy';
import { prefectureRoutes } from './prefecture';
import type { RouteRecordRaw } from 'vue-router';

export const infrastructureRoutes: RouteRecordRaw[] = [
  {
    component: () => import('@/scenes/game/infrastructure/academy/index.vue'),
    path: 'academy',
    children: academyRoutes,
  },
  {
    component: () => import('@/scenes/game/infrastructure/farm/index.vue'),
    path: 'farm',
    name: 'farm' satisfies InfrastructureScene,
  },
  {
    component: () => import('@/scenes/game/infrastructure/iron-mine/index.vue'),
    path: 'iron-mine',
    name: 'iron-mine' satisfies InfrastructureScene,
  },
  {
    component: () => import('@/scenes/game/infrastructure/prefecture/index.vue'),
    path: 'prefecture',
    children: prefectureRoutes,
  },
  {
    component: () => import('@/scenes/game/infrastructure/quarry/index.vue'),
    path: 'quarry',
    name: 'quarry' satisfies InfrastructureScene,
  },
  {
    component: () => import('@/scenes/game/infrastructure/sawmill/index.vue'),
    path: 'sawmill',
    name: 'sawmill' satisfies InfrastructureScene,
  },
  {
    component: () => import('@/scenes/game/infrastructure/silo/index.vue'),
    path: 'silo',
    name: 'silo' satisfies InfrastructureScene,
  },
  {
    component: () => import('@/scenes/game/infrastructure/stable/index.vue'),
    path: 'stable',
    children: stableRoutes,
  },
  {
    component: () => import('@/scenes/game/infrastructure/wall/index.vue'),
    path: 'wall',
    name: 'wall' satisfies InfrastructureScene,
  },
  {
    component: () => import('@/scenes/game/infrastructure/warehouse/index.vue'),
    path: 'warehouse',
    name: 'warehouse' satisfies InfrastructureScene,
  },
];
