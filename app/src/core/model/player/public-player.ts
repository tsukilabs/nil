// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { go } from '@/router';
import * as commands from '@/commands';
import { formatInt } from '@/lib/intl';
import { PLACEHOLDER } from '@/lib/string';
import { RulerImpl } from '@/core/model/ruler';

export class PublicPlayerImpl extends RulerImpl implements PublicPlayer {
  public readonly id: PlayerId;
  public readonly status: PlayerStatus;

  protected constructor(args: PublicPlayerImplConstructorArgs) {
    super({ coords: args.coords, ranking: args.ranking });
    this.id = args.player.id;
    this.status = args.player.status;
  }

  public hasCity(key: ContinentKey) {
    return this.coords.some((it) => it.is(key));
  }

  public getCity(key: ContinentKey) {
    return this.coords.find((it) => it.is(key));
  }

  public isActive() {
    return this.status === 'active';
  }

  public isInactive() {
    return this.status === 'inactive';
  }

  public async goToProfile() {
    await go('profile-player', { params: { id: this.id } });
  }

  public formatRank() {
    return this.ranking?.rank ? formatInt(this.ranking.rank) : PLACEHOLDER;
  }

  public formatScore() {
    return this.ranking?.score ? formatInt(this.ranking.score) : PLACEHOLDER;
  }

  public static create(args: PublicPlayerImplConstructorArgs) {
    if (args.player instanceof PublicPlayerImpl) {
      return args.player;
    }

    return new PublicPlayerImpl(args);
  }

  public static async load(id: PlayerId) {
    const [player, coords, ranking] = await Promise.all([
      commands.getPublicPlayer(id),
      commands.getPlayerCoords(id),
      commands.getPlayerRank(id),
    ]);

    return PublicPlayerImpl.create({ player, coords, ranking });
  }
}

export interface PublicPlayerImplConstructorArgs {
  player: PublicPlayer;
  coords: readonly Coord[];
  ranking: Option<RankingEntry>;
}
