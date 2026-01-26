// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';

export async function startClient(
  serverAddr: ServerAddr,
  worldId: Option<WorldId>,
  playerOptions: PlayerOptions,
) {
  await invoke('start_client', {
    serverAddr,
    worldId: worldId ?? null,
    playerId: playerOptions.id,
    password: playerOptions.password ?? null,
  });
}

export async function stopClient() {
  await invoke('stop_client');
}
