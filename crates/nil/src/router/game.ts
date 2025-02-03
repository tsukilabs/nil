import type { GameRoute } from './types';
import type { RouteRecordRaw } from 'vue-router';

export const gameRoutes: RouteRecordRaw[] = [
  {
    path: 'academy',
    name: 'academy' satisfies GameRoute,
    component: () => import('@/scenes/game/Academy.vue'),
  },
  {
    path: 'farm',
    name: 'farm' satisfies GameRoute,
    component: () => import('@/scenes/game/Farm.vue'),
  },
  {
    path: 'iron-mine',
    name: 'iron-mine' satisfies GameRoute,
    component: () => import('@/scenes/game/IronMine.vue'),
  },
  {
    path: 'prefecture',
    name: 'prefecture' satisfies GameRoute,
    component: () => import('@/scenes/game/Prefecture.vue'),
  },
  {
    path: 'quarry',
    name: 'quarry' satisfies GameRoute,
    component: () => import('@/scenes/game/Quarry.vue'),
  },
  {
    path: 'sawmill',
    name: 'sawmill' satisfies GameRoute,
    component: () => import('@/scenes/game/Sawmill.vue'),
  },
  {
    path: 'silo',
    name: 'silo' satisfies GameRoute,
    component: () => import('@/scenes/game/Silo.vue'),
  },
  {
    path: 'stable',
    name: 'stable' satisfies GameRoute,
    component: () => import('@/scenes/game/Stable.vue'),
  },
  {
    path: 'village',
    name: 'village' satisfies GameRoute,
    component: () => import('@/scenes/game/Village.vue'),
  },
  {
    path: 'wall',
    name: 'wall' satisfies GameRoute,
    component: () => import('@/scenes/game/Wall.vue'),
  },
  {
    path: 'warehouse',
    name: 'warehouse' satisfies GameRoute,
    component: () => import('@/scenes/game/Warehouse.vue'),
  },
];
