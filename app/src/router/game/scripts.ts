// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { RouteRecordRaw } from 'vue-router';

export const scriptsRoutes: RouteRecordRaw[] = [
  {
    component: () => import('@/scenes/game/scripts/root/index.vue'),
    path: '',
    name: 'scripts' satisfies ScriptsScene,
  },
  {
    component: () => import('@/scenes/game/scripts/terminal/index.vue'),
    path: 'terminal',
    name: 'scripts-terminal' satisfies ScriptsScene,
  },
];
