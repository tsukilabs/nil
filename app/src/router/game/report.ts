// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { RouteRecordRaw } from 'vue-router';
import type { ReportScene } from '@/types/scene/game';

export const reportRoutes: RouteRecordRaw[] = [
  {
    component: () => import('@/scenes/game/report/root/index.vue'),
    path: '',
    name: 'report' satisfies ReportScene,
  },
  {
    component: () => import('@/scenes/game/report/forward/index.vue'),
    path: 'forward/:id',
    name: 'report-forward' satisfies ReportScene,
  },
  {
    component: () => import('@/scenes/game/report/view/index.vue'),
    path: 'view/:id',
    name: 'report-view' satisfies ReportScene,
  },
];
