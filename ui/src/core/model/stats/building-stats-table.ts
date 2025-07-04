// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

export class BuildingStatsTableImpl implements BuildingStatsTable {
  public readonly id: BuildingId;
  public readonly minLevel: BuildingLevel;
  public readonly maxLevel: BuildingLevel;
  public readonly table: ReadonlyMap<BuildingLevel, BuildingStats>;

  private constructor(args: BuildingStatsTable) {
    this.id = args.id;
    this.minLevel = args.minLevel;
    this.maxLevel = args.maxLevel;
    this.table = args.table;
  }

  public get(level: BuildingLevel) {
    return this.table.get(level);
  }

  public static fromRaw(raw: RawBuildingStatsTable) {
    const table = new Map<number, BuildingStats>();
    for (const [level, stats] of Object.entries(raw.table)) {
      table.set(Number.parseInt(level), stats);
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
