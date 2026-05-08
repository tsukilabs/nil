// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { WallStatsImpl } from '@/core/model/stats/wall-stats';
import type { BuildingLevel, WallStatsTable } from '@tsukilabs/nil-bindings';

export class WallStatsTableImpl {
  public readonly table: ReadonlyMap<BuildingLevel, WallStatsImpl>;

  private constructor(args: WallStatsTableImplConstructorArgs) {
    this.table = args.table;
  }

  public get(level: BuildingLevel) {
    return this.table.get(level);
  }

  public static fromRaw(raw: WallStatsTable) {
    const table = new Map<number, WallStatsImpl>();
    for (const [level, stats] of Object.entries(raw)) {
      table.set(Number.parseInt(level), WallStatsImpl.create(stats));
    }

    return new WallStatsTableImpl({ table });
  }
}

interface WallStatsTableImplConstructorArgs extends WallStatsTable {
  readonly table: ReadonlyMap<number, WallStatsImpl>;
}
