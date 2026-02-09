// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { Entity } from './abstract';
import { panic } from '@tb-dev/utils';
import { asyncRef, maybe } from '@tb-dev/vue';
import { readonly, ref, type Ref, type ShallowRef } from 'vue';
import type { WorldConfigImpl } from '@/core/model/world-config';
import type { WorldStatsImpl } from '@/core/model/stats/world-stats';
import { getContinentSize, getWorldConfig, getWorldStats } from '@/commands';

export class WorldEntity extends Entity {
  private readonly id = ref<Option<WorldId>>();
  private readonly config: ShallowRef<Option<WorldConfigImpl>>;
  private readonly stats: ShallowRef<Option<WorldStatsImpl>>;
  private readonly continentSize: Ref<number>;

  public override readonly requireManualUpdates = true;

  public readonly updateConfig: () => Promise<void>;
  public readonly updateStats: () => Promise<void>;
  public readonly updateContinentSize: () => Promise<void>;

  constructor() {
    super();

    const config = asyncRef(null, async () => {
      return maybe(this.id, (id) => getWorldConfig(id));
    });

    const stats = asyncRef(null, async () => {
      return maybe(this.id, (id) => getWorldStats(id));
    });

    const continentSize = asyncRef(0, async () => {
      return maybe(this.id, () => getContinentSize()) ?? 0;
    });

    this.config = config.state;
    this.updateConfig = config.load;
    this.stats = stats.state;
    this.updateStats = stats.load;
    this.continentSize = continentSize.state;
    this.updateContinentSize = continentSize.load;
  }

  public override async update() {
    await Promise.all([this.updateConfig(), this.updateStats()]);
    await this.updateContinentSize();
  }

  public static use() {
    return super.get(WorldEntity) as WorldEntity;
  }

  public static refs() {
    const instance = this.use();
    return {
      config: instance.config as Readonly<typeof instance.config>,
      continentSize: readonly(instance.continentSize),
      stats: instance.stats as Readonly<typeof instance.stats>,
      worldId: readonly(instance.id),
    } as const;
  }

  public static update() {
    return this.use().update();
  }

  public static getConfig() {
    return this.use().config.value ?? null;
  }

  public static getId() {
    return this.use().id.value ?? null;
  }

  public static async setId(id?: Option<WorldId>) {
    const world = this.use();
    world.id.value = id;
    await world.update();
  }

  public static getIdStrict() {
    return this.getId() ?? panic('Missing world id');
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

  public static getBuildingStatsWithLevel(id: BuildingId, level: BuildingLevel) {
    return this.getBuildingStats(id)?.get(level) ?? null;
  }

  public static getMineStats(id: MineId) {
    return this.getInfrastructureStats()?.getMine(id) ?? null;
  }

  public static getMineStatsWithLevel(id: MineId, level: BuildingLevel) {
    return this.getMineStats(id)?.get(level) ?? null;
  }

  public static getStorageStats(id: StorageId) {
    return this.getInfrastructureStats()?.getStorage(id) ?? null;
  }

  public static getStorageStatsWithLevel(id: StorageId, level: BuildingLevel) {
    return this.getStorageStats(id)?.get(level) ?? null;
  }

  public static getWallStats() {
    return this.getInfrastructureStats()?.wall ?? null;
  }

  public static getWallStatsWithLevel(level: BuildingLevel) {
    return this.getWallStats()?.get(level) ?? null;
  }

  public static init() {
    if (!Object.hasOwn(globalThis.NIL, 'world')) {
      const world: (typeof globalThis.NIL)['world'] = {
        getBuildingStats: WorldEntity.getBuildingStats.bind(WorldEntity),
        getBuildingStatsWithLevel: WorldEntity.getBuildingStatsWithLevel.bind(WorldEntity),
        getConfig: WorldEntity.getConfig.bind(WorldEntity),
        getContinentSize: WorldEntity.getContinentSize.bind(WorldEntity),
        getId: WorldEntity.getId.bind(WorldEntity),
        getIdStrict: WorldEntity.getIdStrict.bind(WorldEntity),
        getInfrastructureStats: WorldEntity.getInfrastructureStats.bind(WorldEntity),
        getMineStats: WorldEntity.getMineStats.bind(WorldEntity),
        getMineStatsWithLevel: WorldEntity.getMineStatsWithLevel.bind(WorldEntity),
        getStats: WorldEntity.getStats.bind(WorldEntity),
        getStorageStats: WorldEntity.getStorageStats.bind(WorldEntity),
        getStorageStatsWithLevel: WorldEntity.getStorageStatsWithLevel.bind(WorldEntity),
        getWallStats: WorldEntity.getWallStats.bind(WorldEntity),
        getWallStatsWithLevel: WorldEntity.getWallStatsWithLevel.bind(WorldEntity),
        refs: WorldEntity.refs.bind(WorldEntity),
        setId: WorldEntity.setId.bind(WorldEntity),
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
