// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { Option } from "@tb-dev/utils";
import { CoordImpl } from "@/core/model/continent/coord";
import type { ContinentKey } from "@/types/core/continent";
import { RankingEntryImpl } from "@/core/model/ranking/ranking-entry";
import { PublicCapitalImpl } from "@/core/model/capital/public-capital";
import type { Coord, PublicCapital, RankingEntry, Ruler } from "@tsukilabs/nil-bindings";

export abstract class RulerImpl {
  public readonly capital: PublicCapitalImpl;
  public readonly coords: readonly CoordImpl[];
  public readonly ranking: Option<RankingEntryImpl>;

  public abstract readonly toRuler: () => Ruler;

  protected constructor(args: RulerImplConstructorArgs) {
    this.capital = PublicCapitalImpl.create(args.capital);
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
  capital: PublicCapital;
  coords: readonly Coord[];
  ranking: Option<RankingEntry>;
}
