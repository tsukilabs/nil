// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';

export * from './npc';
export * from './nsr';
export * from './chat';
export * from './city';
export * from './cheat';
export * from './round';
export * from './world';
export * from './script';
export * from './client';
export * from './server';
export * from './player';
export * from './ranking';
export * from './continent';
export * from './infrastructure';

export function createTrayIcon() {
  return invoke<null>('create_tray_icon');
}

export function isHost() {
  return invoke<boolean>('is_host');
}

export function showWindow() {
  return invoke<null>('show_window');
}
