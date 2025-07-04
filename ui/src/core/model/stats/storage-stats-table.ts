// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

export class StorageStatsTableImpl implements StorageStatsTable {
  public readonly id: StorageId;
  public readonly table: ReadonlyMap<BuildingLevel, StorageStats>;

  private constructor(args: StorageStatsTable) {
    this.id = args.id;
    this.table = args.table;
  }

  public get(level: BuildingLevel) {
    return this.table.get(level);
  }

  public static fromRaw(raw: RawStorageStatsTable) {
    const table = new Map<number, StorageStats>();
    for (const [level, stats] of Object.entries(raw.table)) {
      table.set(Number.parseInt(level), stats);
    }

    return new StorageStatsTableImpl({ id: raw.id, table });
  }
}

export type RawStorageStatsTable = {
  readonly id: StorageId;
  readonly table: Record<string, StorageStats>;
};
