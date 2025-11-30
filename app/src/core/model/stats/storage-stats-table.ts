// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { StorageStatsImpl } from '@/core/model/stats/storage-stats';

export class StorageStatsTableImpl implements StorageStatsTable {
  public readonly id: StorageId;
  public readonly table: ReadonlyMap<BuildingLevel, StorageStatsImpl>;

  private constructor(args: StorageStatsTableImplConstructorArgs) {
    this.id = args.id;
    this.table = args.table;
  }

  public get(level: BuildingLevel) {
    return this.table.get(level);
  }

  public static fromRaw(raw: RawStorageStatsTable) {
    const table = new Map<number, StorageStatsImpl>();
    for (const [level, stats] of Object.entries(raw.table)) {
      table.set(Number.parseInt(level), StorageStatsImpl.create(stats));
    }

    return new StorageStatsTableImpl({ id: raw.id, table });
  }
}

export interface RawStorageStatsTable {
  readonly id: StorageId;
  readonly table: Record<string, StorageStats>;
}

interface StorageStatsTableImplConstructorArgs extends StorageStatsTable {
  readonly table: ReadonlyMap<number, StorageStatsImpl>;
}
