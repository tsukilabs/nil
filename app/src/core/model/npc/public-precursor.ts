// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { go } from '@/router';
import * as commands from '@/commands';
import { formatInt } from '@/lib/intl';
import { PLACEHOLDER } from '@/lib/string';
import { RulerImpl } from '@/core/model/ruler';

export class PublicPrecursorImpl extends RulerImpl implements PublicPrecursor {
  public readonly id: PrecursorId;
  public readonly origin: Coord;

  public readonly toRuler = (): Ruler => {
    return { kind: 'precursor', id: this.id };
  };

  protected constructor(args: PublicPrecursorImplConstructorArgs) {
    super({ coords: args.coords, ranking: args.ranking });
    this.id = args.precursor.id;
    this.origin = args.precursor.origin;
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
