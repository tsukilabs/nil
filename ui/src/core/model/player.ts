// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import * as commands from '@/commands';
import { ResourcesImpl } from './resource';
import { CoordImpl } from '@/core/model/coord';
import type { PartialNullish } from '@tb-dev/utils';

export class PlayerImpl implements Player {
  public readonly id: string;
  public readonly resources: ResourcesImpl;
  public readonly status: PlayerStatus;
  public readonly coords: readonly CoordImpl[];

  private constructor(player: Player, coords: readonly Coord[]) {
    this.id = player.id;
    this.resources = ResourcesImpl.create(player.resources);
    this.status = player.status;
    this.coords = coords.map((it) => CoordImpl.create(it));
  }

  public hasResources(resources: PartialNullish<Resources>) {
    return this.resources.has(resources);
  }

  public hasVillage(coord: Coord) {
    return this.coords.some((it) => it.is(coord));
  }

  public isGuest() {
    return this.status === 'guest';
  }

  public isActive() {
    return this.status === 'active';
  }

  public isInactive() {
    return this.status === 'inactive';
  }

  public static async load(id: PlayerId) {
    const [player, coords] = await Promise.all([
      commands.getPlayer(id),
      commands.getPlayerCoords(id),
    ]);

    return new PlayerImpl(player, coords);
  }
}
