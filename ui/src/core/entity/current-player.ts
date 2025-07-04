// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { Entity } from './abstract';
import type { Option } from '@tb-dev/utils';
import { asyncRef, maybe } from '@tb-dev/vue';
import { computed, ref, type Ref } from 'vue';
import { PlayerImpl } from '@/core/model/player';

export class CurrentPlayerEntity extends Entity {
  private readonly id = ref<Option<PlayerId>>();
  private readonly player: Ref<Option<PlayerImpl>>;
  private readonly resources: Ref<Option<Resources>>;

  private readonly updatePlayer: () => Promise<void>;

  constructor() {
    super();

    const player = asyncRef(null, async () => {
      return maybe(this.id, (id) => PlayerImpl.load(id));
    });

    this.player = player.state;
    this.updatePlayer = player.execute;

    this.resources = computed(() => {
      return this.player.value?.resources;
    });

    this.initListeners();
  }

  protected override initListeners() {
    this.watch(this.id, this.update.bind(this));
    this.event
      .onPlayerResourcesUpdated(this.update.bind(this))
      .onVillageSpawned(this.onVillageSpawned.bind(this));
  }

  public override async update() {
    await this.updatePlayer();
  }

  private async onVillageSpawned({ village }: VillageSpawnedPayload) {
    if (village.owner.kind === 'player' && village.owner.id === this.id.value) {
      await this.update();
    }
  }

  public static use() {
    return super.get(CurrentPlayerEntity) as CurrentPlayerEntity;
  }

  public static refs() {
    const instance = this.use();
    return {
      id: instance.id as Readonly<typeof instance.id>,
      player: instance.player as Readonly<typeof instance.player>,
      resources: instance.resources as Readonly<typeof instance.resources>,
    } as const;
  }

  public static update() {
    return this.use().update();
  }

  public static setId(id: PlayerId) {
    this.use().id.value = id;
  }

  public static init() {
    if (!Object.hasOwn(window.NIL, 'player')) {
      const player: (typeof window.NIL)['player'] = {
        refs: CurrentPlayerEntity.refs.bind(CurrentPlayerEntity),
        setId: CurrentPlayerEntity.setId.bind(CurrentPlayerEntity),
        update: CurrentPlayerEntity.update.bind(CurrentPlayerEntity),
        use: CurrentPlayerEntity.use.bind(CurrentPlayerEntity),
      };

      Object.defineProperty(window.NIL, 'player', {
        configurable: false,
        enumerable: true,
        value: player,
        writable: false,
      });
    }
  }
}
