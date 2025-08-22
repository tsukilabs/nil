// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

export class MineStatsImpl implements MineStats {
  public readonly level: BuildingLevel;
  public readonly production: number;

  private constructor(stats: MineStats) {
    this.level = stats.level;
    this.production = stats.production;
  }

  public static create(stats: MineStats) {
    return new MineStatsImpl(stats);
  }
}
