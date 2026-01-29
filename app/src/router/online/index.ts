// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { RouteRecordRaw } from 'vue-router';

export const QUERY_SIGN_IN_USER = 'user';
export const QUERY_SIGN_UP_USER = 'user';

export const onlineRoutes: RouteRecordRaw[] = [
  {
    component: () => import('@/scenes/online/lobby/index.vue'),
    path: 'lobby',
    name: 'lobby' satisfies OnlineScene,
  },
  {
    component: () => import('@/scenes/online/sign-in/index.vue'),
    path: 'sign-in',
    name: 'sign-in' satisfies OnlineScene,
  },
  {
    component: () => import('@/scenes/online/sign-up/index.vue'),
    path: 'sign-up',
    name: 'sign-up' satisfies OnlineScene,
  },
];
