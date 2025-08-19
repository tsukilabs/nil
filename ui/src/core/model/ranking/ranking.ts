// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { getRanking } from '@/commands';
import { RankingEntryImpl } from './ranking-entry';

export class RankingImpl {
  public readonly ranking: readonly RankingEntryImpl[];

  private constructor(ranking: Ranking) {
    this.ranking = ranking.map((it) => RankingEntryImpl.create(it));
  }

  public *[Symbol.iterator]() {
    yield* this.ranking;
  }

  public static create(ranking: Ranking) {
    return new RankingImpl(ranking);
  }

  public static async load() {
    const ranking = await getRanking();
    return RankingImpl.create(ranking);
  }
}
