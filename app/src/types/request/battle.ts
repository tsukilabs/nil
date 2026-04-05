// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { Luck } from '@/types/core/battle';
import type { WorldId } from '@/types/core/world';
import type { Squad } from '@/types/core/military/squad';
import type { BuildingLevel } from '@/types/core/infrastructure/building';

export interface SimulateBattleRequest {
  readonly world: WorldId;
  readonly attacker: readonly Squad[];
  readonly defender: readonly Squad[];
  readonly luck: Luck;
  readonly wall: BuildingLevel;
}
