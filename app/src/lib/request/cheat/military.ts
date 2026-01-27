// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

export interface CheatSpawnPersonnelRequest {
  readonly world: WorldId;
  readonly coord: Coord;
  readonly personnel: ArmyPersonnel;
  readonly ruler: Option<Ruler>;
}
