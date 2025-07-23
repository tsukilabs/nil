// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { Entity } from './abstract';
import type { Option } from '@tb-dev/utils';
import { asyncRef, maybe } from '@tb-dev/vue';
import { VillageImpl } from '@/core/model/village';
import { CoordImpl } from '@/core/model/continent/coord';
import { computed, nextTick, type Ref, shallowRef } from 'vue';

export class VillageEntity extends Entity {
  private readonly coord = shallowRef<Option<CoordImpl>>();
  private readonly village: Ref<VillageImpl | null>;
  private readonly production: Ref<Option<Partial<Resources>>>;

  public readonly updateVillage: () => Promise<void>;

  constructor() {
    super();

    const village = asyncRef(null, async () => {
      return maybe(this.coord, (coord) => VillageImpl.load(coord));
    });

    this.village = village.state;
    this.updateVillage = village.execute;

    this.production = computed(() => {
      return this.village.value?.getProduction();
    });

    this.initListeners();
  }

  protected override initListeners() {
    this.event.onVillageUpdated(this.onVillageUpdated.bind(this));
  }

  public override async update() {
    await this.updateVillage();
  }

  public isCoord(coord: Coord) {
    return this.village.value?.coord.is(coord) ?? false;
  }

  private async onVillageUpdated(payload: VillageUpdatedPayload) {
    if (this.isCoord(payload.coord)) {
      await this.update();
    }
  }

  get academy() {
    return this.village.value?.infrastructure.academy;
  }

  get farm() {
    return this.village.value?.infrastructure.farm;
  }

  get ironMine() {
    return this.village.value?.infrastructure.ironMine;
  }

  get prefecture() {
    return this.village.value?.infrastructure.prefecture;
  }

  get quarry() {
    return this.village.value?.infrastructure.quarry;
  }

  get sawmill() {
    return this.village.value?.infrastructure.sawmill;
  }

  get silo() {
    return this.village.value?.infrastructure.silo;
  }

  get stable() {
    return this.village.value?.infrastructure.stable;
  }

  get wall() {
    return this.village.value?.infrastructure.wall;
  }

  get warehouse() {
    return this.village.value?.infrastructure.warehouse;
  }

  public static use() {
    return super.get(VillageEntity) as VillageEntity;
  }

  public static refs() {
    const instance = this.use();
    return {
      coord: instance.coord as Readonly<typeof instance.coord>,
      production: instance.production as Readonly<typeof instance.production>,
      village: instance.village as Readonly<typeof instance.village>,
    } as const;
  }

  public static async setCoord(coord?: Option<Coord>) {
    const village = this.use();
    if (coord) {
      village.coord.value = CoordImpl.create(coord);
    } else {
      await nextTick();
      const { player } = NIL.player.refs();
      village.coord.value = player.value?.coords.at(0);
    }

    await village.update();
  }

  public static init() {
    if (!Object.hasOwn(window.NIL, 'village')) {
      const village: (typeof window.NIL)['village'] = {
        refs: VillageEntity.refs.bind(VillageEntity),
        setCoord: VillageEntity.setCoord.bind(VillageEntity),
        use: VillageEntity.use.bind(VillageEntity),
      };

      Object.defineProperty(window.NIL, 'village', {
        configurable: false,
        enumerable: true,
        writable: false,
        value: village,
      });
    }
  }
}
