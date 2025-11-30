// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';
import { SocketAddrV4 } from '@/lib/net/addr-v4';

export async function getServerAddr() {
  const addr = await invoke<string>('get_server_addr');
  return SocketAddrV4.parse(addr);
}

export function getServerVersion() {
  return invoke<string>('get_server_version');
}

export function isServerReady() {
  return invoke<boolean>('is_server_ready');
}

export async function startServerWithOptions(worldOptions: WorldOptions) {
  const addr = await invoke<string>('start_server_with_options', { worldOptions });
  return SocketAddrV4.parse(addr);
}

export async function startServerWithSavedata(savedata: string) {
  const addr = await invoke<string>('start_server_with_savedata', { savedata });
  return SocketAddrV4.parse(addr);
}

export function stopServer() {
  return invoke<null>('stop_server');
}
