// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import * as commands from '@/commands';
import { PublicVillageImpl } from './public';
import { InfrastructureImpl } from '@/core/model/infrastructure';

export class VillageImpl extends PublicVillageImpl implements Village {
  public readonly infrastructure: InfrastructureImpl;
  public readonly stability: number;

  private constructor(village: Village, infrastructure: InfrastructureImpl) {
    super(village);

    this.infrastructure = infrastructure;
    this.stability = village.stability;
  }

  public getProduction() {
    return {
      food: this.farm.getProduction(),
      iron: this.ironMine.getProduction(),
      stone: this.quarry.getProduction(),
      wood: this.sawmill.getProduction(),
    } satisfies Partial<Resources>;
  }

  get academy() {
    return this.infrastructure.academy;
  }

  get farm() {
    return this.infrastructure.farm;
  }

  get ironMine() {
    return this.infrastructure.ironMine;
  }

  get prefecture() {
    return this.infrastructure.prefecture;
  }

  get quarry() {
    return this.infrastructure.quarry;
  }

  get sawmill() {
    return this.infrastructure.sawmill;
  }

  get silo() {
    return this.infrastructure.silo;
  }

  get stable() {
    return this.infrastructure.stable;
  }

  get wall() {
    return this.infrastructure.wall;
  }

  get warehouse() {
    return this.infrastructure.warehouse;
  }

  public static async load(coord: Coord) {
    const village = await commands.getVillage(coord);
    const infrastructure = InfrastructureImpl.create(village.infrastructure);
    return new VillageImpl(village, infrastructure);
  }
}
