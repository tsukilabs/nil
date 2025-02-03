import type { GameRoute } from './types';
import type { RouteRecordRaw } from 'vue-router';

export const gameRoutes: RouteRecordRaw[] = [
  {
    path: 'village',
    name: 'village' satisfies GameRoute,
    component: () => import('@/scenes/game/Village.vue'),
  },
];
