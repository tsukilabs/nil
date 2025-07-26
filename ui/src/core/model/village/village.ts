// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import * as commands from '@/commands';
import { PublicVillageImpl } from './public-village';
import { CoordImpl } from '@/core/model/continent/coord';
import { InfrastructureImpl } from '@/core/model/infrastructure/infrastructure';

export class VillageImpl extends PublicVillageImpl implements Village {
  public readonly infrastructure: InfrastructureImpl;
  public readonly stability: number;

  private constructor(village: Village) {
    super(village);

    this.infrastructure = InfrastructureImpl.create(village.infrastructure);
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

  public static override create(village: Village) {
    return new VillageImpl(village);
  }

  public static override async load(key: ContinentKey) {
    const coord = CoordImpl.fromKey(key);
    const village = await commands.getVillage(coord);
    return VillageImpl.create(village);
  }
}
