// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';

export * from './chat';
export * from './cheat';
export * from './round';
export * from './world';
export * from './lobby';
export * from './script';
export * from './client';
export * from './server';
export * from './player';
export * from './village';
export * from './continent';
export * from './infrastructure';

export function isDev() {
  return invoke<boolean>('is_dev');
}

export function isHost() {
  return invoke<boolean>('is_host');
}

export function showWindow() {
  return invoke<null>('show_window');
}
