// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { Entity } from './abstract';
import { ref, type Ref } from 'vue';
import { asyncRef } from '@tb-dev/vue';
import type { Option } from '@tb-dev/utils';
import { PlayerImpl } from '@/core/model/player/player';

export class PlayerEntity extends Entity {
  private readonly id = ref<Option<PlayerId>>();
  private readonly player: Ref<Option<PlayerImpl>>;

  public readonly updatePlayer: () => Promise<void>;

  constructor() {
    super();

    const player = usePlayer(this.id);
    this.player = player.state;
    this.updatePlayer = player.execute;

    this.initListeners();
  }

  protected override initListeners() {
    this.event.onPlayerUpdated(this.onPlayerUpdated.bind(this));
  }

  public override async update() {
    await this.updatePlayer();
  }

  private async onPlayerUpdated({ player }: PlayerUpdatedPayload) {
    if (player === this.id.value) {
      await this.update();
    }
  }

  public static use() {
    return super.get(PlayerEntity) as PlayerEntity;
  }

  public static refs() {
    const instance = this.use();
    return {
      id: instance.id as Readonly<typeof instance.id>,
      player: instance.player as Readonly<typeof instance.player>,
    } as const;
  }

  public static update() {
    return this.use().update();
  }

  public static getCoords() {
    return this.use().player.value?.coords ?? [];
  }

  public static getId() {
    return this.use().id.value;
  }

  public static async setId(id?: Option<PlayerId>) {
    const player = this.use();
    player.id.value = id;
    await player.update();
  }

  public static init() {
    if (!Object.hasOwn(globalThis.NIL, 'player')) {
      const player: (typeof globalThis.NIL)['player'] = {
        getCoords: PlayerEntity.getCoords.bind(PlayerEntity),
        getId: PlayerEntity.getId.bind(PlayerEntity),
        refs: PlayerEntity.refs.bind(PlayerEntity),
        setId: PlayerEntity.setId.bind(PlayerEntity),
        update: PlayerEntity.update.bind(PlayerEntity),
        use: PlayerEntity.use.bind(PlayerEntity),
      };

      Object.defineProperty(globalThis.NIL, 'player', {
        configurable: false,
        enumerable: true,
        writable: false,
        value: player,
      });
    }
  }
}

function usePlayer(id: Ref<Option<PlayerId>>) {
  return asyncRef(null, async () => {
    return id.value ? PlayerImpl.load(id.value) : null;
  });
}
