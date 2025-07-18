// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import * as commands from '@/commands';
import type { PartialNullish } from '@tb-dev/utils';
import { ResourcesImpl } from '@/core/model/resource';
import { CoordImpl } from '@/core/model/continent/coord';
import { PlayerStorageCapacityImpl } from './storage-capacity';

export class PlayerImpl implements Player {
  public readonly id: string;
  public readonly status: PlayerStatus;
  public readonly coords: readonly CoordImpl[];
  public readonly resources: ResourcesImpl;
  public readonly capacity: PlayerStorageCapacityImpl;

  private constructor(args: {
    id: string;
    status: PlayerStatus;
    coords: readonly CoordImpl[];
    resources: ResourcesImpl;
    capacity: PlayerStorageCapacityImpl;
  }) {
    this.id = args.id;
    this.status = args.status;
    this.coords = args.coords;
    this.resources = args.resources;
    this.capacity = args.capacity;
  }

  public hasResources(resources: PartialNullish<Resources>) {
    return this.resources.has(resources);
  }

  public hasVillage(coord: Coord) {
    return this.coords.some((it) => it.is(coord));
  }

  public isActive() {
    return this.status === 'active';
  }

  public isInactive() {
    return this.status === 'inactive';
  }

  public static async load(id: PlayerId) {
    const [player, coords, capacity] = await Promise.all([
      commands.getPlayer(id),
      commands.getPlayerCoords(id),
      commands.getPlayerStorageCapacity(),
    ]);

    return new PlayerImpl({
      id: player.id,
      resources: ResourcesImpl.create(player.resources),
      status: player.status,
      coords: coords.map((it) => CoordImpl.create(it)),
      capacity: PlayerStorageCapacityImpl.create(capacity),
    });
  }
}
