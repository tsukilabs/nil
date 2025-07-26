// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

type BuildingId =
  | 'academy'
  | 'farm'
  | 'iron-mine'
  | 'prefecture'
  | 'quarry'
  | 'sawmill'
  | 'silo'
  | 'stable'
  | 'wall'
  | 'warehouse';

type BuildingLevel = number;

interface Building {
  readonly enabled: boolean;
  readonly level: BuildingLevel;
}

type Sawmill = Building;
type Quarry = Building;
type IronMine = Building;
type Farm = Building;
type Warehouse = Building;
type Silo = Building;
type Wall = Building;

interface BuildingStats {
  readonly level: BuildingLevel;
  readonly cost: number;
  readonly resources: Resources;
  readonly maintenance: number;
  readonly workforce: number;
}

interface BuildingStatsTable {
  readonly id: BuildingId;
  readonly minLevel: BuildingLevel;
  readonly maxLevel: BuildingLevel;
  readonly table: ReadonlyMap<BuildingLevel, BuildingStats>;
}
