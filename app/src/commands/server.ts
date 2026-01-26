// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';

export async function getLocalServerWorldId() {
  const serverKind = await getServerKind();
  return serverKind.kind === 'local' ? serverKind.id : null;
}

export async function getServerAddr() {
  return invoke<ServerAddr>('get_server_addr');
}

export async function getServerKind() {
  return invoke<ServerKind>('get_server_kind');
}

export async function getServerVersion() {
  return invoke<string>('get_server_version');
}

export async function isServerReady() {
  return invoke<boolean>('is_server_ready');
}

export async function startServerWithOptions(worldOptions: WorldOptions) {
  return invoke<LocalServer>('start_server_with_options', { worldOptions });
}

export async function startServerWithSavedata(savedata: string) {
  return invoke<LocalServer>('start_server_with_savedata', { savedata });
}

export async function stopServer() {
  await invoke('stop_server');
}
