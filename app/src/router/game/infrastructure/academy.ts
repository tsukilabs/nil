// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { RouteRecordRaw } from 'vue-router';
import type { AcademyScene } from '@/types/scene/game/infrastructure';

export const academyRoutes: RouteRecordRaw[] = [
  {
    component: () => import('@/scenes/game/infrastructure/academy/root/index.vue'),
    path: '',
    name: 'academy' satisfies AcademyScene,
  },
  {
    component: () => import('@/scenes/game/infrastructure/academy/settings/index.vue'),
    path: 'settings',
    name: 'academy-settings' satisfies AcademyScene,
  },
];
