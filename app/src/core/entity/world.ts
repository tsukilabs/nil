// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { Entity } from './abstract';
import { asyncRef } from '@tb-dev/vue';
import { readonly, type Ref, type ShallowRef } from 'vue';
import type { WorldConfigImpl } from '@/core/model/world-config';
import type { WorldStatsImpl } from '@/core/model/stats/world-stats';
import { getContinentSize, getWorldConfig, getWorldStats } from '@/commands';

export class WorldEntity extends Entity {
  private readonly config: ShallowRef<Option<WorldConfigImpl>>;
  private readonly stats: ShallowRef<Option<WorldStatsImpl>>;
  private readonly continentSize: Ref<number>;

  public override readonly requireManualUpdates = true;

  constructor() {
    super();

    this.config = asyncRef(null, getWorldConfig).state;
    this.stats = asyncRef(null, getWorldStats).state;
    this.continentSize = asyncRef(0, getContinentSize).state;
  }

  public static use() {
    return super.get(WorldEntity) as WorldEntity;
  }

  public static refs() {
    const instance = this.use();
    return {
      config: instance.config as Readonly<typeof instance.config>,
      stats: instance.stats as Readonly<typeof instance.stats>,
      continentSize: readonly(instance.continentSize),
    } as const;
  }

  public static getConfig() {
    return this.use().config.value ?? null;
  }

  public static getContinentSize() {
    return this.use().continentSize.value;
  }

  public static getStats() {
    return this.use().stats.value ?? null;
  }

  public static getInfrastructureStats() {
    return this.getStats()?.infrastructure ?? null;
  }

  public static getBuildingStats(id: BuildingId) {
    return this.getInfrastructureStats()?.getBuilding(id) ?? null;
  }

  public static getMineStats(id: MineId) {
    return this.getInfrastructureStats()?.getMine(id) ?? null;
  }

  public static getStorageStats(id: StorageId) {
    return this.getInfrastructureStats()?.getStorage(id) ?? null;
  }

  public static getWallStats() {
    return this.getInfrastructureStats()?.wall ?? null;
  }

  public static init() {
    if (!Object.hasOwn(globalThis.NIL, 'world')) {
      const world: (typeof globalThis.NIL)['world'] = {
        getBuildingStats: WorldEntity.getBuildingStats.bind(WorldEntity),
        getConfig: WorldEntity.getConfig.bind(WorldEntity),
        getContinentSize: WorldEntity.getContinentSize.bind(WorldEntity),
        getInfrastructureStats: WorldEntity.getInfrastructureStats.bind(WorldEntity),
        getMineStats: WorldEntity.getMineStats.bind(WorldEntity),
        getStats: WorldEntity.getStats.bind(WorldEntity),
        getStorageStats: WorldEntity.getStorageStats.bind(WorldEntity),
        getWallStats: WorldEntity.getWallStats.bind(WorldEntity),
        refs: WorldEntity.refs.bind(WorldEntity),
        use: WorldEntity.use.bind(WorldEntity),
      };

      Object.defineProperty(globalThis.NIL, 'world', {
        configurable: false,
        enumerable: true,
        writable: false,
        value: world,
      });
    }
  }
}
