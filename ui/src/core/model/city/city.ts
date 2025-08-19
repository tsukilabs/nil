// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import * as commands from '@/commands';
import { CoordImpl } from '@/core/model/continent/coord';
import { InfrastructureImpl } from '@/core/model/infrastructure/infrastructure';
import { PublicCityImpl, type PublicCityImplConstructorArgs } from './public-city';

export class CityImpl extends PublicCityImpl implements City {
  public readonly infrastructure: InfrastructureImpl;
  public readonly stability: number;

  private constructor(args: CityImplConstructorArgs) {
    super(args);

    this.infrastructure = InfrastructureImpl.create(args.city.infrastructure);
    this.stability = args.city.stability;
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

  public static override create(args: CityImplConstructorArgs) {
    if (args.city instanceof CityImpl) {
      return args.city;
    }

    return new CityImpl(args);
  }

  public static override async load(key: ContinentKey) {
    const coord = CoordImpl.fromKey(key);
    const [city, score] = await Promise.all([
      commands.getCity(coord),
      commands.getCityScore(coord),
    ]);

    return CityImpl.create({ city, score });
  }
}

export interface CityImplConstructorArgs extends PublicCityImplConstructorArgs {
  city: City;
}
