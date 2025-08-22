// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { ResourcesImpl } from '@/core/model/resources';

export class BuildingStatsImpl implements BuildingStats {
  public readonly level: BuildingLevel;
  public readonly cost: number;
  public readonly resources: ResourcesImpl;
  public readonly maintenance: number;
  public readonly workforce: number;

  private constructor(stats: BuildingStats) {
    this.level = stats.level;
    this.cost = stats.cost;
    this.resources = ResourcesImpl.create(stats.resources);
    this.maintenance = stats.maintenance;
    this.workforce = stats.workforce;
  }

  public static create(stats: BuildingStats) {
    return new BuildingStatsImpl(stats);
  }
}
