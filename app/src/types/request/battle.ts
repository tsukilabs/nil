// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

export interface SimulateBattleRequest {
  readonly world: WorldId;
  readonly attacker: readonly Squad[];
  readonly defender: readonly Squad[];
  readonly luck: Luck;
  readonly wall: BuildingLevel;
}
