// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { BuildingLevel } from '@/types/core/infrastructure/building';

export type MineId = 'farm' | 'iron-mine' | 'quarry' | 'sawmill';

export interface MineStats {
  readonly level: BuildingLevel;
  readonly production: number;
}

export interface MineStatsTable {
  readonly id: MineId;
  readonly table: ReadonlyMap<BuildingLevel, MineStats>;
}
