// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { BuildingId, BuildingLevel, BuildingStats } from '@/types/bindings';

export interface Building {
  readonly enabled: boolean;
  readonly level: BuildingLevel;
}

export interface BuildingStatsTable {
  readonly id: BuildingId;
  readonly minLevel: BuildingLevel;
  readonly maxLevel: BuildingLevel;
  readonly table: ReadonlyMap<BuildingLevel, BuildingStats>;
}
