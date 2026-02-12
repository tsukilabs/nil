// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { RouteRecordRaw } from 'vue-router';

export const continentRoutes: RouteRecordRaw[] = [
  {
    component: () => import('@/scenes/game/continent/root/index.vue'),
    path: '',
    name: 'continent' satisfies ContinentScene,
  },
  {
    component: () => import('@/scenes/game/continent/cities/index.vue'),
    path: 'cities',
    name: 'continent-cities' satisfies ContinentScene,
  },
];
