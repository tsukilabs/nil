// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { RouteRecordRaw } from 'vue-router';

export const academyRoutes: RouteRecordRaw[] = [
  {
    component: () => import('@/scenes/game/infrastructure/academy/root/index.vue'),
    path: '',
    name: 'academy' satisfies AcademyScene,
  },
  {
    component: () => import('@/scenes/game/infrastructure/academy/settings/index.vue'),
    path: 'academy-settings',
    name: 'academy-settings' satisfies AcademyScene,
  },
];
