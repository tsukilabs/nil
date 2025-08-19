// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { Entity } from './abstract';
import type { ShallowRef } from 'vue';
import { asyncRef } from '@tb-dev/vue';
import { RankingImpl } from '@/core/model/ranking/ranking';

export class RankingEntity extends Entity {
  private readonly ranking: ShallowRef<Option<RankingImpl>>;

  public readonly updateRanking: () => Promise<void>;

  constructor() {
    super();

    const ranking = asyncRef(null, RankingImpl.load.bind(RankingImpl));
    this.ranking = ranking.state;
    this.updateRanking = ranking.execute;

    this.initListeners();
  }

  public override async update() {
    await this.updateRanking();
  }

  public static use() {
    return super.get(RankingEntity) as RankingEntity;
  }

  public static refs() {
    const instance = this.use();
    return {
      ranking: instance.ranking as Readonly<typeof instance.ranking>,
    } as const;
  }

  public static update() {
    return this.use().update();
  }

  public static init() {
    if (!Object.hasOwn(globalThis.NIL, 'ranking')) {
      const ranking: (typeof globalThis.NIL)['ranking'] = {
        refs: RankingEntity.refs.bind(RankingEntity),
        update: RankingEntity.update.bind(RankingEntity),
        use: RankingEntity.use.bind(RankingEntity),
      };

      Object.defineProperty(globalThis.NIL, 'ranking', {
        configurable: false,
        enumerable: true,
        writable: false,
        value: ranking,
      });
    }
  }
}
