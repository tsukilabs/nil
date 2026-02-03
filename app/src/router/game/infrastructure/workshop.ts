// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { RouteRecordRaw } from 'vue-router';

export const workshopRoutes: RouteRecordRaw[] = [
  {
    component: () => import('@/scenes/game/infrastructure/workshop/root/index.vue'),
    path: '',
    name: 'workshop' satisfies WorkshopScene,
  },
  {
    component: () => import('@/scenes/game/infrastructure/workshop/settings/index.vue'),
    path: 'settings',
    name: 'workshop-settings' satisfies WorkshopScene,
  },
];
