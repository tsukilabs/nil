// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { go } from '@/router';
import * as commands from '@/commands';
import { CoordImpl } from '@/core/model/continent/coord';

export class PublicPlayerImpl implements PublicPlayer {
  public readonly id: PlayerId;
  public readonly status: PlayerStatus;
  public readonly coords: readonly CoordImpl[];

  protected constructor(args: PublicPlayerImplConstructorArgs) {
    this.id = args.player.id;
    this.status = args.player.status;
    this.coords = args.coords.map((it) => CoordImpl.create(it));
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

  public static create(args: PublicPlayerImplConstructorArgs) {
    if (args.player instanceof PublicPlayerImpl) {
      return args.player;
    }

    return new PublicPlayerImpl(args);
  }

  public static async load(id: PlayerId) {
    const [player, coords] = await Promise.all([
      commands.getPublicPlayer(id),
      commands.getPlayerCoords(id),
    ]);

    return PublicPlayerImpl.create({ player, coords });
  }
}

export interface PublicPlayerImplConstructorArgs {
  player: PublicPlayer;
  coords: readonly Coord[];
}
