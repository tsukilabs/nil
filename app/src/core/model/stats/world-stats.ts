// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { BuildingId, WorldStats } from '@tsukilabs/nil-bindings';
import { InfrastructureStatsImpl } from './infrastructure-stats';

export class WorldStatsImpl {
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

  public static fromRaw(raw: WorldStats) {
    const infrastructure = InfrastructureStatsImpl.fromRaw(raw.infrastructure);
    return new WorldStatsImpl({ infrastructure });
  }
}
