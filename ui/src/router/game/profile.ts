// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { RouteRecordRaw } from 'vue-router';

export const profileRoutes: RouteRecordRaw[] = [
  {
    component: () => import('@/scenes/game/profile/village/index.vue'),
    path: 'profile/village/:ckey',
    name: 'profile-village' satisfies ProfileScene,
  },
];
