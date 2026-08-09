// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import * as commands from "@/commands";
import type { PartialNullish } from "@tb-dev/utils";
import type { DeepReadonly } from "es-toolkit/types";
import { ResourcesImpl } from "@/core/model/resources";
import { CapitalImpl } from "@/core/model/capital/capital";
import { PublicPlayerImpl, type PublicPlayerImplConstructorArgs } from "./public-player";
import { OverallStorageCapacityImpl } from "@/core/model/infrastructure/storage-capacity";
import type {
  Gold,
  Influence,
  OverallStorageCapacity,
  Player,
  PlayerId,
  Resources,
} from "@tsukilabs/nil-bindings";

export class PlayerImpl extends PublicPlayerImpl implements DeepReadonly<Player> {
  public override readonly capital: CapitalImpl;
  public readonly resources: ResourcesImpl;
  public readonly gold: Gold;
  public readonly influence: Influence;
  public readonly capacity: OverallStorageCapacityImpl;

  private constructor(args: PlayerImplConstructorArgs) {
    super(args);
    this.capital = CapitalImpl.create(args.player.capital);
    this.resources = ResourcesImpl.create(args.player.resources);
    this.gold = args.player.gold;
    this.influence = args.player.influence;
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
      commands.getPlayer(),
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
