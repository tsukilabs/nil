// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { Entity } from './abstract';
import type { ShallowRef } from 'vue';
import { asyncRef } from '@tb-dev/vue';
import { getWorldConfig, getWorldStats } from '@/commands';
import type { WorldConfigImpl } from '@/core/model/world-config';
import type { WorldStatsImpl } from '@/core/model/stats/world-stats';

export class WorldEntity extends Entity {
  private readonly config: ShallowRef<Option<WorldConfigImpl>>;
  private readonly stats: ShallowRef<Option<WorldStatsImpl>>;

  constructor() {
    super();

    this.config = asyncRef(null, getWorldConfig).state;
    this.stats = asyncRef(null, getWorldStats).state;
  }

  public static use() {
    return super.get(WorldEntity) as WorldEntity;
  }

  public static refs() {
    const instance = this.use();
    return {
      config: instance.config as Readonly<typeof instance.config>,
      stats: instance.stats as Readonly<typeof instance.stats>,
    } as const;
  }

  public static init() {
    if (!Object.hasOwn(window.NIL, 'world')) {
      const world: (typeof window.NIL)['world'] = {
        refs: WorldEntity.refs.bind(WorldEntity),
        use: WorldEntity.use.bind(WorldEntity),
      };

      Object.defineProperty(window.NIL, 'world', {
        configurable: false,
        enumerable: true,
        writable: false,
        value: world,
      });
    }
  }
}
