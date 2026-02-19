// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';
import type { CheatGetPlayersRequest } from '@/lib/request/cheat/player';

export async function cheatGetPlayers() {
  const req: CheatGetPlayersRequest = {
    world: NIL.world.getIdStrict(),
  };

  return invoke<readonly Player[]>('cheat_get_players', { req });
}
