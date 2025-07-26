// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { getPublicVillage } from '@/commands';
import { CoordImpl } from '@/core/model/continent/coord';

export class PublicVillageImpl implements PublicVillage {
  public readonly coord: CoordImpl;
  public readonly name: string;
  public readonly owner: VillageOwner;

  protected constructor(village: PublicVillage) {
    this.coord = CoordImpl.create(village.coord);
    this.name = village.name;
    this.owner = village.owner;
  }

  public static create(village: PublicVillage) {
    return new PublicVillageImpl(village);
  }

  public static async load(key: ContinentKey) {
    const coord = CoordImpl.fromKey(key);
    const village = await getPublicVillage(coord);
    return PublicVillageImpl.create(village);
  }
}
