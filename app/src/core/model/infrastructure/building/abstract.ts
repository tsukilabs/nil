// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { Option } from '@tb-dev/utils';

export abstract class BuildingImpl implements Building {
  public abstract readonly id: BuildingId;
  public readonly level: BuildingLevel;
  public readonly enabled: boolean;

  #minLevel: Option<BuildingLevel> = null;
  #maxLevel: Option<BuildingLevel> = null;

  protected constructor(building: Building) {
    this.level = building.level;
    this.enabled = building.enabled;
  }

  public getStatsTable() {
    const { stats } = NIL.world.refs();
    return stats.value?.infrastructure.getBuilding(this.id);
  }

  public getStats() {
    return this.getStatsBy(this.level);
  }

  public getStatsBy(level: BuildingLevel) {
    return this.getStatsTable()?.get(level);
  }

  public getCost() {
    return this.getCostBy(this.level);
  }

  public getCostBy(level: BuildingLevel) {
    return this.getStatsBy(level)?.cost;
  }

  public getMaintenance() {
    return this.getMaintenanceBy(this.level);
  }

  public getMaintenanceBy(level: BuildingLevel) {
    return this.getStatsBy(level)?.maintenance;
  }

  public getResourceCost() {
    return this.getResourceCostBy(this.level);
  }

  public getResourceCostBy(level: BuildingLevel) {
    return this.getStatsBy(level)?.resources;
  }

  public getWorkforce() {
    return this.getWorkforceBy(this.level);
  }

  public getWorkforceBy(level: BuildingLevel) {
    return this.getStatsBy(level)?.workforce;
  }

  public isMaxLevel() {
    return this.level >= this.maxLevel;
  }

  get minLevel() {
    if (typeof this.#minLevel !== 'number') {
      this.#minLevel = this.getStatsTable()?.minLevel;
    }

    return this.#minLevel ?? 0;
  }

  get maxLevel() {
    if (typeof this.#maxLevel !== 'number') {
      this.#maxLevel = this.getStatsTable()?.maxLevel;
    }

    return this.#maxLevel ?? 0;
  }
}

export abstract class MineImpl extends BuildingImpl {
  public abstract override readonly id: MineId;

  public getMineStatsTable() {
    const { stats } = NIL.world.refs();
    return stats.value?.infrastructure.getMine(this.id);
  }

  public getMineStats() {
    return this.getMineStatsBy(this.level);
  }

  public getMineStatsBy(level: BuildingLevel) {
    return this.getMineStatsTable()?.get(level);
  }

  public getProduction() {
    return this.getProductionBy(this.level);
  }

  public getProductionBy(level: BuildingLevel) {
    return this.getMineStatsBy(level)?.production;
  }
}

export abstract class StorageImpl extends BuildingImpl {
  public abstract override readonly id: StorageId;

  public getStorageStatsTable() {
    const { stats } = NIL.world.refs();
    return stats.value?.infrastructure.getStorage(this.id);
  }

  public getStorageStats() {
    return this.getStorageStatsBy(this.level);
  }

  public getStorageStatsBy(level: BuildingLevel) {
    return this.getStorageStatsTable()?.get(level);
  }

  public getCapacity() {
    return this.getCapacityBy(this.level);
  }

  public getCapacityBy(level: BuildingLevel) {
    return this.getStorageStatsBy(level)?.capacity;
  }
}
