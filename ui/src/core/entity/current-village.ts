// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { go } from '@/router';
import { Entity } from './abstract';
import { until } from '@vueuse/core';
import { asyncRef, maybe } from '@tb-dev/vue';
import { CoordImpl } from '@/core/model/coord';
import { VillageImpl } from '@/core/model/village';
import { type Option, sleep } from '@tb-dev/utils';
import type { PlayerImpl } from '@/core/model/player';
import { computed, nextTick, type Ref, shallowRef } from 'vue';

/**
 * Depends on:
 * - [`CurrentPlayerEntity`](./current-player.ts)
 */
export class CurrentVillageEntity extends Entity {
  private readonly coord = shallowRef<Option<CoordImpl>>();
  private readonly village: Ref<VillageImpl | null>;
  private readonly production: Ref<Option<Partial<Resources>>>;

  private readonly updateVillage: () => Promise<void>;

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
    const { player } = NIL.player.refs();

    // prettier-ignore
    this
      .watch(this.coord, this.updateVillage)
      .watch(player, this.onPlayerUpdate.bind(this));

    this.event.onPrefectureBuildQueueUpdated(this.onPrefectureBuildQueueUpdated.bind(this));
  }

  public override async update() {
    await this.updateVillage();
  }

  private onPlayerUpdate(player: Option<PlayerImpl>) {
    if (player && (!this.coord.value || !player.hasVillage(this.coord.value))) {
      this.coord.value = player.coords.at(0);
    } else if (!player) {
      this.coord.value = null;
    }
  }

  private async onPrefectureBuildQueueUpdated(payload: PrefectureBuildQueueUpdatedPayload) {
    if (this.village.value?.coord.is(payload.coord)) {
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
    return super.get(CurrentVillageEntity) as CurrentVillageEntity;
  }

  public static refs() {
    const instance = this.use();
    return {
      coord: instance.coord as Readonly<typeof instance.coord>,
      production: instance.production as Readonly<typeof instance.production>,
      village: instance.village,
    } as const;
  }

  public static readonly go = goToVillage;

  public static init() {
    if (!Object.hasOwn(window.NIL, 'village')) {
      const village: (typeof window.NIL)['village'] = {
        go: CurrentVillageEntity.go.bind(CurrentVillageEntity),
        refs: CurrentVillageEntity.refs.bind(CurrentVillageEntity),
        use: CurrentVillageEntity.use.bind(CurrentVillageEntity),
      };

      Object.defineProperty(window.NIL, 'village', {
        configurable: false,
        enumerable: true,
        value: village,
        writable: false,
      });
    }
  }
}

async function goToVillage(options?: {
  timeout?: number;
  keepTrying?: boolean;
  maxTries?: number;
}) {
  const { coord } = NIL.village.refs();
  const { timeout = 5000, keepTrying = false, maxTries = 50 } = options ?? {};

  const wait = async () => {
    if (typeof timeout === 'number' && Number.isFinite(timeout) && timeout > 0) {
      await until(coord).toBeTruthy({ timeout, throwOnTimeout: true });
    } else {
      await until(coord).toBeTruthy();
    }
  };

  const navigate = () => go('village');

  if (keepTrying) {
    let current = 0;
    const { player } = NIL.player.refs();
    const retry = async () => {
      await nextTick();
      if (player.value?.isActive()) {
        try {
          await wait();
          await navigate();
          return;
        } catch {
          await NIL.update();
        }
      } else {
        await sleep(500);
      }

      current += 1;

      if (current < maxTries) {
        await retry();
      }
    };

    await retry();
  } else {
    await navigate();
  }
}
