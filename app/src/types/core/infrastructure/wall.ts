// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { Building, BuildingLevel } from '@/types/core/infrastructure/building';

export type Wall = Building;

export interface WallStats {
  readonly level: BuildingLevel;
  readonly defense: number;
  readonly defensePercent: number;
}

export interface WallStatsTable {
  readonly table: ReadonlyMap<BuildingLevel, WallStats>;
}
