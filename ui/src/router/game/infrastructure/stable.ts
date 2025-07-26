// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { RouteRecordRaw } from 'vue-router';

export const stableRoutes: RouteRecordRaw[] = [
  {
    component: () => import('@/scenes/game/infrastructure/stable/root/index.vue'),
    path: '',
    name: 'stable' satisfies StableScene,
  },
  {
    component: () => import('@/scenes/game/infrastructure/stable/settings/index.vue'),
    path: 'stable-settings',
    name: 'stable-settings' satisfies StableScene,
  },
];
