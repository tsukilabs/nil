// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { Entity } from './abstract';
import { asyncRef } from '@tb-dev/vue';
import type { Option } from '@tb-dev/utils';
import { RoundImpl } from '@/core/model/round';
import { computed, type ComputedRef, type Ref } from 'vue';

/**
 * Depends on:
 * - [`CurrentPlayerEntity`](./current-player.ts)
 */
export class RoundEntity extends Entity {
  private readonly round: Ref<Option<RoundImpl>>;
  private readonly isPlayerTurn: ComputedRef<boolean>;

  private readonly updateRound: () => Promise<void>;

  constructor() {
    super();

    const round = asyncRef(null, () => RoundImpl.load());
    this.round = round.state;
    this.updateRound = round.execute;

    const { player } = NIL.player.refs();
    this.isPlayerTurn = computed(() => {
      const id = player.value?.id;
      const pending = id ? this.round.value?.isPending(id) : null;
      return pending ?? false;
    });

    this.initListeners();
  }

  protected override initListeners() {
    this.event.onRoundUpdated(this.onRoundUpdated.bind(this));
  }

  public override async update() {
    await this.updateRound();
  }

  private async onRoundUpdated({ round }: RoundUpdatedPayload) {
    // Isso geralmente indica que o round atual acabou, então nós atualizamos todas as entidades.
    if (round.id !== this.round.value?.id || round.phase.kind !== this.round.value.phase.kind) {
      await NIL.update();
    } else {
      this.round.value = RoundImpl.create(round);
    }
  }

  public static use() {
    return super.get(RoundEntity) as RoundEntity;
  }

  public static refs() {
    const instance = this.use();
    return {
      isPlayerTurn: instance.isPlayerTurn,
      round: instance.round as Readonly<typeof instance.round>,
    } as const;
  }

  public static update() {
    return this.use().update();
  }

  public static init() {
    if (!Object.hasOwn(window.NIL, 'round')) {
      const round: (typeof window.NIL)['round'] = {
        refs: RoundEntity.refs.bind(RoundEntity),
        update: RoundEntity.update.bind(RoundEntity),
        use: RoundEntity.use.bind(RoundEntity),
      };

      Object.defineProperty(window.NIL, 'round', {
        configurable: false,
        enumerable: true,
        value: round,
        writable: false,
      });
    }
  }
}
