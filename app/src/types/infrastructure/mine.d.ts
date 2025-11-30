// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

type MineId = 'farm' | 'iron-mine' | 'quarry' | 'sawmill';

interface MineStats {
  readonly level: BuildingLevel;
  readonly production: number;
}

interface MineStatsTable {
  readonly id: MineId;
  readonly table: ReadonlyMap<BuildingLevel, MineStats>;
}
