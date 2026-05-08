// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { BuildingStatsImpl } from '@/core/model/stats/building-stats';
import type { BuildingId, BuildingLevel, BuildingStatsTable } from '@tsukilabs/nil-bindings';

type Stats = Omit<Readonly<BuildingStatsTable>, 'table'>;

export class BuildingStatsTableImpl implements Stats {
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

  public static fromRaw(raw: BuildingStatsTable) {
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

interface BuildingStatsTableImplConstructorArgs extends Stats {
  readonly table: ReadonlyMap<number, BuildingStatsImpl>;
}
