// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';

export async function getClientVersion() {
  return invoke<string>('get_client_version');
}

export async function stopClient() {
  await invoke('stop_client');
}

export async function updateClient(options: ClientOptions) {
  if (options.serverAddr.kind !== 'remote') {
    options.worldPassword = null;
    options.playerPassword = null;
    options.authorizationToken = null;
  }

  await invoke('update_client', {
    serverAddr: options.serverAddr,
    worldId: options.worldId ?? null,
    worldPassword: options.worldPassword ?? null,
    playerId: options.playerId ?? null,
    playerPassword: options.playerPassword ?? null,
    authorizationToken: options.authorizationToken ?? null,
  });
}
