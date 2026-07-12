// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import * as commands from "@/commands";
import { CoordImpl } from "@/core/model/continent/coord";
import type { ContinentKey } from "@/types/core/continent";
import type { City, Resources } from "@tsukilabs/nil-bindings";
import { InfrastructureImpl } from "@/core/model/infrastructure/infrastructure";
import { PublicCityImpl, type PublicCityImplConstructorArgs } from "./public-city";

export class CityImpl extends PublicCityImpl implements Readonly<City> {
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

  get workshop() {
    return this.infrastructure.workshop;
  }

  public static override create(args: CityImplConstructorArgs) {
    if (args.city instanceof CityImpl) {
      return args.city;
    }

    return new CityImpl(args);
  }

  public static override async load(key: ContinentKey) {
    const coord = CoordImpl.fromContinentKey(key);
    const { city, score } = await commands.getCity({
      coord,
      score: true,
    });

    return CityImpl.create({ city, score: score ?? 0 });
  }

  public static override async bulkLoad(keys: readonly ContinentKey[]): Promise<CityImpl[]> {
    if (keys.length === 0) {
      return [];
    }

    const response = await commands.getCities({
      coords: keys.map((key) => CoordImpl.fromContinentKey(key)),
      score: true,
    });

    return response.map(({ city, score }) => {
      return CityImpl.create({ city, score: score ?? 0 });
    });
  }
}

export interface CityImplConstructorArgs extends PublicCityImplConstructorArgs {
  city: Readonly<City>;
}
