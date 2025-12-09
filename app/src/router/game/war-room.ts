// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { RouteRecordRaw } from 'vue-router';

export const QUERY_WAR_ROOM_ORIGIN_X = 'originX';
export const QUERY_WAR_ROOM_ORIGIN_Y = 'originY';
export const QUERY_WAR_ROOM_DEST_X = 'destX';
export const QUERY_WAR_ROOM_DEST_Y = 'destY';

export const warRoomRoutes: RouteRecordRaw[] = [
  {
    component: () => import('@/scenes/game/war-room/root/index.vue'),
    path: '',
    name: 'war-room' satisfies WarRoomScene,
  },
  {
    component: () => import('@/scenes/game/war-room/simulator/index.vue'),
    path: 'simulator',
    name: 'war-room-simulator' satisfies WarRoomScene,
  },
];
