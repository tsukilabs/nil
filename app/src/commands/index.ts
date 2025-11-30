// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';

export * from './npc';
export * from './chat';
export * from './city';
export * from './cheat';
export * from './round';
export * from './world';
export * from './client';
export * from './server';
export * from './player';
export * from './ranking';
export * from './continent';
export * from './infrastructure';

export async function createTrayIcon() {
  await invoke('create_tray_icon');
}

export async function isHost() {
  return invoke<boolean>('is_host');
}

export async function showWindow() {
  await invoke('show_window');
}

export async function version() {
  return invoke<string>('version');
}
