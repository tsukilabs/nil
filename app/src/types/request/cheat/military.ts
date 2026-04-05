// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { Ruler } from '@/types/core/ruler';
import type { WorldId } from '@/types/core/world';
import type { Coord } from '@/types/core/continent';
import type { ArmyPersonnel } from '@/types/core/military/army';

export interface CheatGetIdleArmiesAtRequest {
  readonly world: WorldId;
  readonly coord: Coord;
}

export interface CheatGetIdlePersonnelAtRequest {
  readonly world: WorldId;
  readonly coord: Coord;
}

export interface CheatSpawnPersonnelRequest {
  readonly world: WorldId;
  readonly coord: Coord;
  readonly personnel: ArmyPersonnel;
  readonly ruler: Option<Ruler>;
}
