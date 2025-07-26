// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import * as commands from '@/commands';
import { ResourcesImpl } from '@/core/model/resources';
import { CoordImpl } from '@/core/model/continent/coord';
import { OverallStorageCapacityImpl } from '@/core/model/infrastructure/storage-capacity';

export class PlayerImpl implements Player {
  public readonly id: PlayerId;
  public readonly status: PlayerStatus;
  public readonly coords: readonly CoordImpl[];
  public readonly resources: ResourcesImpl;
  public readonly capacity: OverallStorageCapacityImpl;

  private constructor(args: Args) {
    this.id = args.id;
    this.status = args.status;
    this.coords = args.coords;
    this.resources = args.resources;
    this.capacity = args.capacity;
  }

  public hasResources(resources: PartialNullish<Resources>) {
    return this.resources.has(resources);
  }

  public hasVillage(key: ContinentKey) {
    return this.coords.some((it) => it.is(key));
  }

  public getVillage(key: ContinentKey) {
    return this.coords.find((it) => it.is(key));
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
      capacity: OverallStorageCapacityImpl.create(capacity),
    });
  }
}

interface Args {
  id: PlayerId;
  status: PlayerStatus;
  coords: readonly CoordImpl[];
  resources: ResourcesImpl;
  capacity: OverallStorageCapacityImpl;
}
