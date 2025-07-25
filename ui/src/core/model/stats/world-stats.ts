// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { InfrastructureStatsImpl, type RawInfrastructureStats } from './infrastructure-stats';

export class WorldStatsImpl implements WorldStats {
  public readonly infrastructure: InfrastructureStatsImpl;

  private constructor(stats: { infrastructure: InfrastructureStatsImpl; }) {
    this.infrastructure = stats.infrastructure;
  }

  public getBuildingMinLevel(building: BuildingId) {
    return this.infrastructure.getMinLevel(building);
  }

  public getBuildingMaxLevel(building: BuildingId) {
    return this.infrastructure.getMaxLevel(building);
  }

  public static fromRaw(raw: RawWorldStats) {
    const infrastructure = InfrastructureStatsImpl.fromRaw(raw.infrastructure);
    return new WorldStatsImpl({ infrastructure });
  }
}

export interface RawWorldStats extends Omit<WorldStats, 'infrastructure'> {
  infrastructure: RawInfrastructureStats;
}
