// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { BuildingImpl } from './abstract';

export class WallImpl extends BuildingImpl implements Wall {
  public readonly id: BuildingId = 'wall';

  private constructor(wall: Wall) {
    super(wall);
  }

  public getWallStatsTable() {
    const { stats } = NIL.world.refs();
    return stats.value?.infrastructure.wall;
  }

  public getWallStats() {
    return this.getWallStatsBy(this.level);
  }

  public getWallStatsBy(level: BuildingLevel) {
    return this.getWallStatsTable()?.get(level);
  }

  public getDefense() {
    return this.getDefenseBy(this.level);
  }

  public getDefenseBy(level: BuildingLevel) {
    return this.getWallStatsBy(level)?.defense;
  }

  public getDefensePercent() {
    return this.getDefensePercentBy(this.level);
  }

  public getDefensePercentBy(level: BuildingLevel) {
    return this.getWallStatsBy(level)?.defensePercent;
  }

  public static create(wall: Wall) {
    return new WallImpl(wall);
  }
}
