// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';

export async function stopClient() {
  await invoke('stop_client');
}

export async function updateClient(options: {
  serverAddr: ServerAddr;
  worldId?: Option<WorldId>;
  playerId?: Option<PlayerId>;
  playerPassword?: Option<string>;
}) {
  await invoke('update_client', {
    serverAddr: options.serverAddr,
    worldId: options.worldId ?? null,
    playerId: options.playerId ?? null,
    playerPassword: options.playerPassword ?? null,
  });
}
