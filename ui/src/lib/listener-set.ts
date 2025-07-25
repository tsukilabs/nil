// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { tryOnScopeDispose } from '@vueuse/core';
import { type EventProxy, events, type ListenerFn } from '@/lib/event';
import { type Fn, type MaybeArray, type MaybePromise, toArray } from '@tb-dev/utils';
import {
  watch,
  type WatchCallback,
  watchEffect,
  type WatchEffect,
  type WatchEffectOptions,
  type WatchOptions,
  type WatchSource,
} from 'vue';

type Event = {
  [K in keyof EventProxy]: (fn: Parameters<EventProxy[K]>[0]) => Event;
};

export class ListenerSet {
  private readonly set = new Set<Fn>();
  private disposed = false;

  constructor() {
    tryOnScopeDispose(() => this.dispose());
  }

  private async add(listener: MaybePromise<MaybeArray<Fn>>) {
    for (const unlisten of toArray(await listener)) {
      if (this.disposed) {
        unlisten();
      }
      else {
        this.set.add(unlisten);
      }
    }
  }

  public on(fn: MaybePromise<MaybeArray<Fn>>) {
    this.add(fn).err();
    return this;
  }

  public off() {
    this.set.forEach((unlisten) => unlisten());
    this.set.clear();
  }

  public dispose() {
    try {
      this.off();
    }
    finally {
      this.disposed = true;
    }
  }

  public readonly event = new Proxy({} as Event, {
    get: <T extends keyof typeof events>(_: unknown, key: T) => {
      return (cb: ListenerFn<T>) => {
        this.on(events[key](cb as any));
        return this.event;
      };
    },
  });

  public watch<T, Immediate extends Readonly<boolean> = false>(
    source: WatchSource<T>,
    cb: WatchCallback<T>,
    options?: WatchOptions<Immediate>,
  ) {
    return this.on(watch(source, cb, options));
  }

  public watchEffect(effect: WatchEffect, options?: WatchEffectOptions) {
    return this.on(watchEffect(effect, options));
  }
}
