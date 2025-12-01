// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { Ref } from 'vue';
import { Entity } from './abstract';
import { asyncRef } from '@tb-dev/vue';
import { MilitaryImpl } from '@/core/model/military/military';

export class MilitaryEntity extends Entity {
  private readonly military: Ref<Option<MilitaryImpl>>;

  public readonly updateMilitary: () => Promise<void>;

  constructor() {
    super();

    const military = asyncRef(null, MilitaryImpl.load.bind(MilitaryImpl));
    this.military = military.state;
    this.updateMilitary = military.execute;

    this.initListeners();
  }

  protected override initListeners() {
    this.event.onMilitaryUpdated(this.onMilitaryUpdated.bind(this));
  }

  public override async update() {
    await this.updateMilitary();
  }

  private async onMilitaryUpdated({ player }: MilitaryUpdatedPayload) {
    if (player === NIL.player.getId()) {
      await this.update();
    }
  }

  public static use() {
    return super.get(MilitaryEntity) as MilitaryEntity;
  }

  public static refs() {
    const instance = this.use();
    return {
      military: instance.military as Readonly<typeof instance.military>,
    } as const;
  }

  public static update() {
    return this.use().update();
  }

  public static init() {
    if (!Object.hasOwn(globalThis.NIL, 'military')) {
      const military: (typeof globalThis.NIL)['military'] = {
        refs: MilitaryEntity.refs.bind(MilitaryEntity),
        update: MilitaryEntity.update.bind(MilitaryEntity),
        use: MilitaryEntity.use.bind(MilitaryEntity),
      };

      Object.defineProperty(globalThis.NIL, 'military', {
        configurable: false,
        enumerable: true,
        writable: false,
        value: military,
      });
    }
  }
}
