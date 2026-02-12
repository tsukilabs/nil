// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { go } from '@/router';
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

  public async goToOwnerProfile() {
    const scene: ProfileScene = `profile-${this.owner.kind}`;
    await go(scene, { params: { id: this.owner.id } });
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
    const response = await commands.getPublicCity({
      coord: CoordImpl.fromContinentKey(key),
      score: true,
    });

    return PublicCityImpl.create({
      city: response.city,
      score: response.score ?? 0,
    });
  }

  public static async loadAll() {
    const response = await commands.getPublicCities({
      coords: null,
      score: true,
      all: true,
    });

    return response.map(({ city, score }) => {
      return PublicCityImpl.create({ city, score: score ?? 0 });
    });
  }

  public static async bulkLoad(keys: readonly ContinentKey[]) {
    const response = await commands.getPublicCities({
      coords: keys.map((key) => CoordImpl.fromContinentKey(key)),
      score: true,
      all: false,
    });

    return response.map(({ city, score }) => {
      return PublicCityImpl.create({ city, score: score ?? 0 });
    });
  }
}

export interface PublicCityImplConstructorArgs {
  city: PublicCity;
  score: number;
}
