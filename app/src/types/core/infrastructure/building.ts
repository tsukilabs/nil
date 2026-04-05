// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { Resources } from '@/types/core/resources';

export type BuildingId =
  | 'academy'
  | 'farm'
  | 'iron-mine'
  | 'prefecture'
  | 'quarry'
  | 'sawmill'
  | 'silo'
  | 'stable'
  | 'wall'
  | 'warehouse'
  | 'workshop';

export type BuildingLevel = number;
export type BuildingLevelDiff = number;

export interface Building {
  readonly enabled: boolean;
  readonly level: BuildingLevel;
}

export type Sawmill = Building;
export type Quarry = Building;
export type IronMine = Building;
export type Farm = Building;
export type Warehouse = Building;
export type Silo = Building;

export interface BuildingStats {
  readonly level: BuildingLevel;
  readonly cost: number;
  readonly resources: Resources;
  readonly maintenance: number;
  readonly workforce: number;
}

export interface BuildingStatsTable {
  readonly id: BuildingId;
  readonly minLevel: BuildingLevel;
  readonly maxLevel: BuildingLevel;
  readonly table: ReadonlyMap<BuildingLevel, BuildingStats>;
}
