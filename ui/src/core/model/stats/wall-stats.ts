// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

export class WallStatsImpl implements WallStats {
  public readonly level: BuildingLevel;
  public readonly defense: number;
  public readonly defensePercent: number;

  private constructor(stats: WallStats) {
    this.level = stats.level;
    this.defense = stats.defense;
    this.defensePercent = stats.defensePercent;
  }

  public static create(stats: WallStats) {
    return new WallStatsImpl(stats);
  }
}
