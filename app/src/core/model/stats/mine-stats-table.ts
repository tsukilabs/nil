// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { MineStatsImpl } from '@/core/model/stats/mine-stats';

export class MineStatsTableImpl implements MineStatsTable {
  public readonly id: MineId;
  public readonly table: ReadonlyMap<BuildingLevel, MineStatsImpl>;

  private constructor(args: MineStatsTableImplConstructorArgs) {
    this.id = args.id;
    this.table = args.table;
  }

  public get(level: BuildingLevel) {
    return this.table.get(level);
  }

  public static fromRaw(raw: RawMineStatsTable) {
    const table = new Map<number, MineStatsImpl>();
    for (const [level, stats] of Object.entries(raw.table)) {
      table.set(Number.parseInt(level), MineStatsImpl.create(stats));
    }

    return new MineStatsTableImpl({ id: raw.id, table });
  }
}

export interface RawMineStatsTable {
  readonly id: MineId;
  readonly table: Record<string, MineStats>;
}

interface MineStatsTableImplConstructorArgs extends MineStatsTable {
  readonly table: ReadonlyMap<number, MineStatsImpl>;
}
