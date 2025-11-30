// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import * as commands from '@/commands';
import { ResourcesImpl } from '@/core/model/resources';
import { PublicPlayerImpl, type PublicPlayerImplConstructorArgs } from './public-player';
import { OverallStorageCapacityImpl } from '@/core/model/infrastructure/storage-capacity';

export class PlayerImpl extends PublicPlayerImpl implements Player {
  public readonly resources: ResourcesImpl;
  public readonly capacity: OverallStorageCapacityImpl;

  private constructor(args: PlayerImplConstructorArgs) {
    super(args);
    this.resources = ResourcesImpl.create(args.player.resources);
    this.capacity = OverallStorageCapacityImpl.create(args.capacity);
  }

  public hasResources(resources: PartialNullish<Resources>) {
    return this.resources.has(resources);
  }

  public static override create(args: PlayerImplConstructorArgs) {
    if (args.player instanceof PlayerImpl) {
      return args.player;
    }

    return new PlayerImpl(args);
  }

  public static override async load(id: PlayerId) {
    const [player, coords, ranking, capacity] = await Promise.all([
      commands.getPlayer(id),
      commands.getPlayerCoords(id),
      commands.getPlayerRank(id),
      commands.getPlayerStorageCapacity(),
    ]);

    return PlayerImpl.create({ player, coords, ranking, capacity });
  }
}

export interface PlayerImplConstructorArgs extends PublicPlayerImplConstructorArgs {
  player: Player;
  capacity: OverallStorageCapacity;
}
