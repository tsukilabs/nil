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
    return this.use().config.value;
  }

  public static getContinentSize() {
    return this.use().continentSize.value;
  }

  public static getStats() {
    return this.use().stats.value;
  }

  public static init() {
    if (!Object.hasOwn(globalThis.NIL, 'world')) {
      const world: (typeof globalThis.NIL)['world'] = {
        getConfig: WorldEntity.getConfig.bind(WorldEntity),
        getContinentSize: WorldEntity.getContinentSize.bind(WorldEntity),
        getStats: WorldEntity.getStats.bind(WorldEntity),
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
