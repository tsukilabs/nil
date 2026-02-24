// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';
import type { CheatGetPlayerRequest, CheatGetPlayersRequest } from '@/lib/request/cheat/player';

export async function cheatGetPlayer(player?: Option<PlayerId>) {
  const req: CheatGetPlayerRequest = {
    world: NIL.world.getIdStrict(),
    player: player ?? null,
  };

  return invoke<Player>('cheat_get_player', { req });
}

export async function cheatGetPlayers() {
  const req: CheatGetPlayersRequest = {
    world: NIL.world.getIdStrict(),
  };

  return invoke<readonly Player[]>('cheat_get_players', { req });
}
