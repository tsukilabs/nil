// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

interface WallStats {
  readonly level: BuildingLevel;
  readonly defense: number;
  readonly defensePercent: number;
}

interface WallStatsTable {
  readonly table: ReadonlyMap<BuildingLevel, WallStats>;
}
