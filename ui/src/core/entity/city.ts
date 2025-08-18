// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { Entity } from './abstract';
import { asyncRef, maybe } from '@tb-dev/vue';
import { CityImpl } from '@/core/model/city/city';
import { CoordImpl } from '@/core/model/continent/coord';
import { computed, nextTick, type Ref, shallowRef } from 'vue';

export class CityEntity extends Entity {
  private readonly coord = shallowRef<Option<CoordImpl>>();
  private readonly city: Ref<CityImpl | null>;
  private readonly production: Ref<Option<Partial<Resources>>>;

  public readonly updateCity: () => Promise<void>;

  constructor() {
    super();

    const city = asyncRef(null, async () => {
      return maybe(this.coord, (coord) => CityImpl.load(coord));
    });

    this.city = city.state;
    this.updateCity = city.execute;

    this.production = computed(() => {
      return this.city.value?.getProduction();
    });

    this.initListeners();
  }

  protected override initListeners() {
    this.event.onCityUpdated(this.onCityUpdated.bind(this));
  }

  public override async update() {
    await this.updateCity();
  }

  public isCoord(coord: Coord) {
    return this.city.value?.coord.is(coord) ?? false;
  }

  private async onCityUpdated(payload: CityUpdatedPayload) {
    if (this.isCoord(payload.coord)) {
      await this.update();
    }
  }

  get academy() {
    return this.city.value?.infrastructure.academy;
  }

  get farm() {
    return this.city.value?.infrastructure.farm;
  }

  get ironMine() {
    return this.city.value?.infrastructure.ironMine;
  }

  get prefecture() {
    return this.city.value?.infrastructure.prefecture;
  }

  get quarry() {
    return this.city.value?.infrastructure.quarry;
  }

  get sawmill() {
    return this.city.value?.infrastructure.sawmill;
  }

  get silo() {
    return this.city.value?.infrastructure.silo;
  }

  get stable() {
    return this.city.value?.infrastructure.stable;
  }

  get wall() {
    return this.city.value?.infrastructure.wall;
  }

  get warehouse() {
    return this.city.value?.infrastructure.warehouse;
  }

  public static use() {
    return super.get(CityEntity) as CityEntity;
  }

  public static refs() {
    const instance = this.use();
    return {
      coord: instance.coord as Readonly<typeof instance.coord>,
      production: instance.production as Readonly<typeof instance.production>,
      city: instance.city as Readonly<typeof instance.city>,
    } as const;
  }

  public static async setCoord(coord?: Option<Coord>) {
    const city = this.use();
    if (coord) {
      city.coord.value = CoordImpl.create(coord);
    }
    else {
      await nextTick();
      const { player } = NIL.player.refs();
      city.coord.value = player.value?.coords.at(0);
    }

    await city.update();
  }

  public static init() {
    if (!Object.hasOwn(globalThis.NIL, 'city')) {
      const city: (typeof globalThis.NIL)['city'] = {
        refs: CityEntity.refs.bind(CityEntity),
        setCoord: CityEntity.setCoord.bind(CityEntity),
        use: CityEntity.use.bind(CityEntity),
      };

      Object.defineProperty(globalThis.NIL, 'city', {
        configurable: false,
        enumerable: true,
        writable: false,
        value: city,
      });
    }
  }
}
