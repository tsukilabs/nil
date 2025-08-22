// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { BuildingStatsImpl } from '@/core/model/stats/building-stats';

export class BuildingStatsTableImpl implements BuildingStatsTable {
  public readonly id: BuildingId;
  public readonly minLevel: BuildingLevel;
  public readonly maxLevel: BuildingLevel;
  public readonly table: ReadonlyMap<BuildingLevel, BuildingStatsImpl>;

  private constructor(args: BuildingStatsTableImplConstructorArgs) {
    this.id = args.id;
    this.minLevel = args.minLevel;
    this.maxLevel = args.maxLevel;
    this.table = args.table;
  }

  public get(level: BuildingLevel) {
    return this.table.get(level);
  }

  public static fromRaw(raw: RawBuildingStatsTable) {
    const table = new Map<number, BuildingStatsImpl>();
    for (const [level, stats] of Object.entries(raw.table)) {
      table.set(Number.parseInt(level), BuildingStatsImpl.create(stats));
    }

    return new BuildingStatsTableImpl({
      id: raw.id,
      minLevel: raw.minLevel,
      maxLevel: raw.maxLevel,
      table,
    });
  }
}

export interface RawBuildingStatsTable {
  readonly id: BuildingId;
  readonly minLevel: BuildingLevel;
  readonly maxLevel: BuildingLevel;
  readonly table: Record<string, BuildingStats>;
}

interface BuildingStatsTableImplConstructorArgs extends BuildingStatsTable {
  readonly table: ReadonlyMap<number, BuildingStatsImpl>;
}
