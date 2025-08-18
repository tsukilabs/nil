// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { go } from '@/router';
import { getPublicCity } from '@/commands';
import { CoordImpl } from '@/core/model/continent/coord';

export class PublicCityImpl implements PublicCity {
  public readonly coord: CoordImpl;
  public readonly name: string;
  public readonly owner: CityOwner;

  protected constructor(city: PublicCity) {
    this.coord = CoordImpl.create(city.coord);
    this.name = city.name;
    this.owner = city.owner;
  }

  public async goToProfile() {
    const ckey = this.coord.toIndexString();
    await go('profile-city', { params: { ckey } });
  }

  public async goToContinent() {
    await this.coord.goToContinent();
  }

  get index() {
    return this.coord.toIndex();
  }

  public static create(city: PublicCity) {
    if (city instanceof PublicCityImpl) {
      return city;
    }

    return new PublicCityImpl(city);
  }

  public static async load(key: ContinentKey) {
    const coord = CoordImpl.fromKey(key);
    const city = await getPublicCity(coord);
    return PublicCityImpl.create(city);
  }
}
