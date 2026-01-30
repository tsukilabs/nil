// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import * as commands from '@/commands';
import { fromZoned } from '@/lib/date';
import { WorldConfigImpl } from './world-config';

export class RemoteWorldImpl implements RemoteWorld {
  public readonly config: WorldConfigImpl;
  public readonly description: Option<string>;
  public readonly createdBy: PlayerId;
  public readonly createdAt: string;
  public readonly updatedAt: string;
  public readonly hasPassword: boolean;
  public readonly currentRound: RoundId;
  public readonly activePlayers: number;
  public readonly totalPlayers: number;

  #createdAtDate: Option<Date> = null;
  #updatedAtDate: Option<Date> = null;

  private constructor(world: RemoteWorld) {
    this.config = WorldConfigImpl.create(world.config);
    this.description = world.description?.trim();
    this.createdBy = world.createdBy;
    this.createdAt = world.createdAt;
    this.updatedAt = world.updatedAt;
    this.hasPassword = world.hasPassword;
    this.currentRound = world.currentRound;
    this.activePlayers = world.activePlayers;
    this.totalPlayers = world.totalPlayers;
  }

  public wasCreatedBy(player: PlayerId) {
    return this.createdBy === player;
  }

  get id() {
    return this.config.id;
  }

  get name() {
    return this.config.name;
  }

  get createdAtDate() {
    this.#createdAtDate ??= fromZoned(this.createdAt);
    return this.#createdAtDate;
  }

  get updatedAtDate() {
    this.#updatedAtDate ??= fromZoned(this.updatedAt);
    return this.#updatedAtDate;
  }

  public static create(world: RemoteWorld) {
    if (world instanceof RemoteWorldImpl) {
      return world;
    }

    return new RemoteWorldImpl(world);
  }

  public static async load(id?: Option<WorldId>) {
    const world = await commands.getRemoteWorld(id);
    return RemoteWorldImpl.create(world);
  }

  public static async wasCreatedBy(world: WorldId, player: PlayerId) {
    const remoteWorld = await RemoteWorldImpl.load(world);
    return remoteWorld.createdBy === player;
  }
}
