// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import * as commands from '@/commands';
import { formatInt } from '@/lib/intl';
import { CoordImpl } from '@/core/model/continent/coord';

export class PublicCityImpl implements PublicCity {
  public readonly coord: CoordImpl;
  public readonly name: string;
  public readonly owner: Ruler;
  public readonly score: number;

  protected constructor(args: PublicCityImplConstructorArgs) {
    this.coord = CoordImpl.create(args.city.coord);
    this.name = args.city.name;
    this.owner = args.city.owner;
    this.score = args.score;
  }

  public async goToContinent() {
    await this.coord.goToContinent();
  }

  public async goToProfile() {
    await this.coord.goToProfile();
  }

  public async goToWarRoom(kind: 'origin' | 'destination') {
    await this.coord.goToWarRoom(kind);
  }

  public formatScore() {
    return formatInt(this.score);
  }

  get index() {
    return this.coord.toContinentIndex();
  }

  public static create(args: PublicCityImplConstructorArgs) {
    if (args.city instanceof PublicCityImpl) {
      return args.city;
    }

    return new PublicCityImpl(args);
  }

  public static async load(key: ContinentKey) {
    const coord = CoordImpl.fromContinentKey(key);
    const [city, score] = await Promise.all([
      commands.getPublicCity(coord),
      commands.getCityScore(coord),
    ]);

    return PublicCityImpl.create({ city, score });
  }
}

export interface PublicCityImplConstructorArgs {
  city: PublicCity;
  score: number;
}
