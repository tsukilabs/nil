// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { Option } from "@tb-dev/utils";
import { invoke } from "@tauri-apps/api/core";
import type {
  BotId,
  CheatSetInfluenceRequest,
  Influence,
  PlayerId,
  PrecursorId,
  Ruler,
} from "@tsukilabs/nil-bindings";

export async function cheatSetInfluence(ruler: Option<Ruler>, influence: Influence) {
  const req: CheatSetInfluenceRequest = {
    world: NIL.world.getIdStrict(),
    ruler: ruler ?? null,
    influence,
  };

  await invoke("cheat_set_influence", { req });
}

export async function cheatSetBotInfluence(id: BotId, influence: Influence) {
  return cheatSetInfluence({ kind: "bot", id }, influence);
}

export async function cheatSetPlayerInfluence(id: PlayerId, influence: Influence) {
  return cheatSetInfluence({ kind: "player", id }, influence);
}

export async function cheatSetPrecursorInfluence(id: PrecursorId, influence: Influence) {
  return cheatSetInfluence({ kind: "precursor", id }, influence);
}
