// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { RouteRecordRaw } from 'vue-router';

export const profileRoutes: RouteRecordRaw[] = [
  {
    component: () => import('@/scenes/game/profile/bot/index.vue'),
    path: 'bot/:id',
    name: 'profile-bot' satisfies ProfileScene,
  },
  {
    component: () => import('@/scenes/game/profile/city/index.vue'),
    path: 'city/:ckey', // ContinentKey
    name: 'profile-city' satisfies ProfileScene,
  },
  {
    component: () => import('@/scenes/game/profile/player/index.vue'),
    path: 'player/:id',
    name: 'profile-player' satisfies ProfileScene,
  },
  {
    component: () => import('@/scenes/game/profile/precursor/index.vue'),
    path: 'precursor/:id',
    name: 'profile-precursor' satisfies ProfileScene,
  },
];
