// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

/* eslint-disable @typescript-eslint/class-methods-use-this */
import type { Option } from '@tb-dev/utils';
import { asyncNoop, noop } from 'es-toolkit';
import { ListenerSet } from '@/lib/listener-set';
import { type EffectScope, effectScope } from 'vue';

type Ctor = new () => Entity;

export abstract class Entity {
  private static readonly table = new Map<Ctor, Entity>();
  protected static readonly withScope = createScope();

  private static create(ctor: Ctor) {
    return this.withScope(() => {
      // eslint-disable-next-line new-cap
      const instance = new ctor();
      this.table.set(ctor, instance);
      return instance;
    });
  }

  protected static get(ctor: Ctor) {
    return this.table.get(ctor) ?? this.create(ctor);
  }

  public static dispose() {
    this.table.forEach((it) => it.dispose());
    this.table.clear();
  }

  public static async updateAll() {
    await Promise.all(this.table.values().map((it) => it.update()));
  }

  private readonly listeners = new ListenerSet();
  private readonly dispose = this.listeners.dispose.bind(this.listeners);
  protected readonly watch = this.listeners.watch.bind(this.listeners);
  protected readonly watchEffect = this.listeners.watchEffect.bind(this.listeners);

  protected initListeners() {
    noop();
  }

  public update() {
    return asyncNoop();
  }

  protected get event() {
    return this.listeners.event;
  }
}

function createScope() {
  let value: Option<EffectScope>;
  return <T>(fn: () => T) => {
    value ??= effectScope(/* detached */ true);
    return value.run(fn) as T;
  };
}
