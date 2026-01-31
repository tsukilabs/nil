// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';

export * from './npc';
export * from './chat';
export * from './city';
export * from './user';
export * from './cheat';
export * from './round';
export * from './world';
export * from './battle';
export * from './client';
export * from './player';
export * from './report';
export * from './server';
export * from './ranking';
export * from './military';
export * from './continent';
export * from './infrastructure';

export async function clearAllBrowsingData() {
  await invoke('clear_all_browsing_data');
}

export async function createTrayIcon() {
  await invoke('create_tray_icon');
}

export async function isHost() {
  return invoke<boolean>('is_host');
}

export async function isLocal() {
  return invoke<boolean>('is_local');
}

export async function isLocalAndHost() {
  return invoke<boolean>('is_local_and_host');
}

export async function isRemote() {
  return invoke<boolean>('is_remote');
}

export async function isRemoteOrHost() {
  return invoke<boolean>('is_remote_or_host');
}

export async function showWindow() {
  await invoke('show_window');
}

export async function version() {
  return invoke<string>('version');
}
