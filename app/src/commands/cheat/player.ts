// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { Option } from "@tb-dev/utils";
import { invoke } from "@tauri-apps/api/core";
import type {
  CheatGetPlayerRequest,
  CheatGetPlayerResponse,
  CheatGetPlayersRequest,
  CheatGetPlayersResponse,
  PlayerId,
} from "@tsukilabs/nil-bindings";

export async function cheatGetPlayer(player?: Option<PlayerId>) {
  const req: CheatGetPlayerRequest = {
    world: NIL.world.getIdStrict(),
    player: player ?? null,
  };

  return invoke<CheatGetPlayerResponse>("cheat_get_player", { req });
}

export async function cheatGetPlayers() {
  const req: CheatGetPlayersRequest = {
    world: NIL.world.getIdStrict(),
  };

  return invoke<CheatGetPlayersResponse>("cheat_get_players", { req });
}
