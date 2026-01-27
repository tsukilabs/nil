// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

export interface CheatGetEthicsRequest {
  readonly world: WorldId;
  readonly ruler: Ruler;
}

export interface CheatSetBotEthicsRequest {
  readonly world: WorldId;
  readonly id: BotId;
  readonly ethics: Ethics;
}

export interface CheatSpawnBotRequest {
  readonly world: WorldId;
  readonly name: string;
}
