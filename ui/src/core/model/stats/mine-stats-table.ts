// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

export class MineStatsTableImpl implements MineStatsTable {
  public readonly id: MineId;
  public readonly table: ReadonlyMap<BuildingLevel, MineStats>;

  private constructor(args: MineStatsTable) {
    this.id = args.id;
    this.table = args.table;
  }

  public get(level: BuildingLevel) {
    return this.table.get(level);
  }

  public static fromRaw(raw: RawMineStatsTable) {
    const table = new Map<number, MineStats>();
    for (const [level, stats] of Object.entries(raw.table)) {
      table.set(Number.parseInt(level), stats);
    }

    return new MineStatsTableImpl({ id: raw.id, table });
  }
}

export type RawMineStatsTable = {
  readonly id: MineId;
  readonly table: Record<string, MineStats>;
};
