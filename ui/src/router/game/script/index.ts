// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { RouteRecordRaw } from 'vue-router';

export const scriptRoutes: RouteRecordRaw[] = [
  {
    component: () => import('@/scenes/game/script/root/index.vue'),
    path: '',
    name: 'script' satisfies ScriptScene,
  },
  {
    component: () => import('@/scenes/game/script/nsr/index.vue'),
    path: 'nsr',
    name: 'nsr' satisfies ScriptScene,
  },
];
