// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { Ref } from 'vue';
import { Entity } from './abstract';
import { asyncRef } from '@tb-dev/vue';
import { RoundImpl } from '@/core/model/round';

export class RoundEntity extends Entity {
  private readonly round: Ref<Option<RoundImpl>>;

  public readonly updateRound: () => Promise<void>;

  constructor() {
    super();

    const round = asyncRef(null, () => RoundImpl.load());
    this.round = round.state;
    this.updateRound = round.execute;

    this.initListeners();
  }

  protected override initListeners() {
    this.event.onRoundUpdated(this.onRoundUpdated.bind(this));
  }

  public override async update() {
    await this.updateRound();
  }

  private async onRoundUpdated({ round }: RoundUpdatedPayload) {
    // This typically indicates that the current round is complete, so we update all the entities.
    if (round.id !== this.id || round.state.kind !== this.state?.kind) {
      await NIL.update();
    }
    else {
      this.round.value = RoundImpl.create(round);
    }
  }

  get id() {
    return this.round.value?.id;
  }

  get state() {
    return this.round.value?.state;
  }

  public static use() {
    return super.get(RoundEntity) as RoundEntity;
  }

  public static refs() {
    const instance = this.use();
    return {
      round: instance.round as Readonly<typeof instance.round>,
    } as const;
  }

  public static update() {
    return this.use().update();
  }

  public static init() {
    if (!Object.hasOwn(globalThis.NIL, 'round')) {
      const round: (typeof globalThis.NIL)['round'] = {
        refs: RoundEntity.refs.bind(RoundEntity),
        update: RoundEntity.update.bind(RoundEntity),
        use: RoundEntity.use.bind(RoundEntity),
      };

      Object.defineProperty(globalThis.NIL, 'round', {
        configurable: false,
        enumerable: true,
        writable: false,
        value: round,
      });
    }
  }
}
