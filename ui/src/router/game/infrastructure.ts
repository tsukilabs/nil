// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { RouteRecordRaw } from 'vue-router';

export const infrastructureRoutes: RouteRecordRaw[] = [
  {
    component: () => import('@/scenes/game/infrastructure/academy/index.vue'),
    name: 'academy' satisfies InfrastructureScene,
    path: 'academy',
  },
  {
    component: () => import('@/scenes/game/infrastructure/farm/index.vue'),
    name: 'farm' satisfies InfrastructureScene,
    path: 'farm',
  },
  {
    component: () => import('@/scenes/game/infrastructure/iron-mine/index.vue'),
    name: 'iron-mine' satisfies InfrastructureScene,
    path: 'iron-mine',
  },
  {
    component: () => import('@/scenes/game/infrastructure/prefecture/index.vue'),
    name: 'prefecture' satisfies InfrastructureScene,
    path: 'prefecture',
  },
  {
    component: () => import('@/scenes/game/infrastructure/quarry/index.vue'),
    name: 'quarry' satisfies InfrastructureScene,
    path: 'quarry',
  },
  {
    component: () => import('@/scenes/game/infrastructure/sawmill/index.vue'),
    name: 'sawmill' satisfies InfrastructureScene,
    path: 'sawmill',
  },
  {
    component: () => import('@/scenes/game/infrastructure/silo/index.vue'),
    name: 'silo' satisfies InfrastructureScene,
    path: 'silo',
  },
  {
    component: () => import('@/scenes/game/infrastructure/stable/index.vue'),
    name: 'stable' satisfies InfrastructureScene,
    path: 'stable',
  },
  {
    component: () => import('@/scenes/game/infrastructure/wall/index.vue'),
    name: 'wall' satisfies InfrastructureScene,
    path: 'wall',
  },
  {
    component: () => import('@/scenes/game/infrastructure/warehouse/index.vue'),
    name: 'warehouse' satisfies InfrastructureScene,
    path: 'warehouse',
  },
];
