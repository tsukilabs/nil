// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { Option } from '@tb-dev/utils';
import { CoordImpl } from '@/core/model/continent/coord';
import type { ContinentKey } from '@/types/core/continent';
import type { Coord, RankingEntry, Ruler } from '@tsukilabs/nil-bindings';
import { RankingEntryImpl } from '@/core/model/ranking/ranking-entry';

export abstract class RulerImpl {
  public readonly coords: readonly CoordImpl[];
  public readonly ranking: Option<RankingEntryImpl>;

  public abstract readonly toRuler: () => Ruler;

  protected constructor(args: RulerImplConstructorArgs) {
    this.coords = args.coords.map((it) => CoordImpl.create(it));

    if (args.ranking) {
      this.ranking = RankingEntryImpl.create(args.ranking);
    }
  }

  public owns(key: ContinentKey) {
    const coord = CoordImpl.fromContinentKey(key);
    return this.coords.some((it) => it.is(coord));
  }
}

export interface RulerImplConstructorArgs {
  coords: readonly Coord[];
  ranking: Option<RankingEntry>;
}
