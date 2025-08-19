// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { go } from '@/router';
import * as commands from '@/commands';
import { formatInt } from '@/lib/intl';
import { PLACEHOLDER } from '@/lib/string';
import { CoordImpl } from '@/core/model/continent/coord';
import { RankingEntryImpl } from '@/core/model/ranking/ranking-entry';

export class PublicPrecursorImpl implements PublicPrecursor {
  public readonly id: PrecursorId;
  public readonly origin: Coord;
  public readonly coords: readonly CoordImpl[];
  public readonly ranking: Option<RankingEntryImpl>;

  protected constructor(args: PublicPrecursorImplConstructorArgs) {
    this.id = args.precursor.id;
    this.origin = args.precursor.origin;
    this.coords = args.coords.map((it) => CoordImpl.create(it));

    if (args.ranking) {
      this.ranking = RankingEntryImpl.create(args.ranking);
    }
  }

  public hasCity(key: ContinentKey) {
    return this.coords.some((it) => it.is(key));
  }

  public getCity(key: ContinentKey) {
    return this.coords.find((it) => it.is(key));
  }

  public async goToProfile() {
    await go('profile-precursor', { params: { id: this.id } });
  }

  public formatRank() {
    return this.ranking?.rank ? formatInt(this.ranking.rank) : PLACEHOLDER;
  }

  public formatScore() {
    return this.ranking?.score ? formatInt(this.ranking.score) : PLACEHOLDER;
  }

  public static create(args: PublicPrecursorImplConstructorArgs) {
    if (args.precursor instanceof PublicPrecursorImpl) {
      return args.precursor;
    }

    return new PublicPrecursorImpl(args);
  }

  public static async load(id: PrecursorId) {
    const [precursor, coords, ranking] = await Promise.all([
      commands.getPublicPrecursor(id),
      commands.getPrecursorCoords(id),
      commands.getPrecursorRank(id),
    ]);

    return PublicPrecursorImpl.create({ precursor, coords, ranking });
  }
}

export interface PublicPrecursorImplConstructorArgs {
  precursor: PublicPrecursor;
  coords: readonly Coord[];
  ranking: Option<RankingEntry>;
}
