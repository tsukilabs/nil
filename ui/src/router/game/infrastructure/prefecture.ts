// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { RouteRecordRaw } from 'vue-router';

export const prefectureRoutes: RouteRecordRaw[] = [
  {
    component: () => import('@/scenes/game/infrastructure/prefecture/root/index.vue'),
    path: '',
    name: 'prefecture' satisfies PrefectureScene,
  },
  {
    component: () => import('@/scenes/game/infrastructure/prefecture/settings/index.vue'),
    path: 'prefecture-settings',
    name: 'prefecture-settings' satisfies PrefectureScene,
  },
];
