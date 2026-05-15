// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from "@tauri-apps/api/core";
import type {
  GetRoundRequest,
  GetRoundResponse,
  SetPlayerReadyRequest,
  SetPlayerReadyResponse,
  StartRoundRequest,
  StartRoundResponse,
} from "@tsukilabs/nil-bindings";

export async function getRound() {
  const req: GetRoundRequest = {
    world: NIL.world.getIdStrict(),
  };

  return invoke<GetRoundResponse>("get_round", { req });
}

export async function isRoundIdle() {
  const req: GetRoundRequest = {
    world: NIL.world.getIdStrict(),
  };

  return invoke<boolean>("is_round_idle", { req });
}

export async function isRoundWaiting() {
  const req: GetRoundRequest = {
    world: NIL.world.getIdStrict(),
  };

  return invoke<boolean>("is_round_waiting", { req });
}

export async function setPlayerReady(isReady: boolean) {
  const req: SetPlayerReadyRequest = {
    world: NIL.world.getIdStrict(),
    isReady,
  };

  return invoke<SetPlayerReadyResponse>("set_player_ready", { req });
}

export async function startRound() {
  const req: StartRoundRequest = {
    world: NIL.world.getIdStrict(),
  };

  return invoke<StartRoundResponse>("start_round", { req });
}
