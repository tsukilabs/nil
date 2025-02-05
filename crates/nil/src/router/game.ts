import type { GameRoute } from './types';
import type { RouteRecordRaw } from 'vue-router';

export const gameRoutes: RouteRecordRaw[] = [
  {
    component: () => import('@/scenes/game/academy/index.vue'),
    name: 'academy' satisfies GameRoute,
    path: 'academy',
  },
  {
    component: () => import('@/scenes/game/farm/index.vue'),
    name: 'farm' satisfies GameRoute,
    path: 'farm',
  },
  {
    component: () => import('@/scenes/game/iron-mine/index.vue'),
    name: 'iron-mine' satisfies GameRoute,
    path: 'iron-mine',
  },
  {
    component: () => import('@/scenes/game/prefecture/index.vue'),
    name: 'prefecture' satisfies GameRoute,
    path: 'prefecture',
  },
  {
    component: () => import('@/scenes/game/quarry/index.vue'),
    name: 'quarry' satisfies GameRoute,
    path: 'quarry',
  },
  {
    component: () => import('@/scenes/game/sawmill/index.vue'),
    name: 'sawmill' satisfies GameRoute,
    path: 'sawmill',
  },
  {
    component: () => import('@/scenes/game/silo/index.vue'),
    name: 'silo' satisfies GameRoute,
    path: 'silo',
  },
  {
    component: () => import('@/scenes/game/stable/index.vue'),
    name: 'stable' satisfies GameRoute,
    path: 'stable',
  },
  {
    component: () => import('@/scenes/game/village/index.vue'),
    name: 'village' satisfies GameRoute,
    path: 'village',
  },
  {
    component: () => import('@/scenes/game/wall/index.vue'),
    name: 'wall' satisfies GameRoute,
    path: 'wall',
  },
  {
    component: () => import('@/scenes/game/warehouse/index.vue'),
    name: 'warehouse' satisfies GameRoute,
    path: 'warehouse',
  },
];
