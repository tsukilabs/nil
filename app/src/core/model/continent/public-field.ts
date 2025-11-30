// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { getField, getFields } from '@/commands';
import { tryOnScopeDispose } from '@vueuse/core';
import type { MaybePromise } from '@tb-dev/utils';
import type { CoordImpl } from '@/core/model/continent/coord';

const enum Flags {
  Uninit = 1 << 0,
  Loading = 1 << 1,
  Empty = 1 << 2,
  City = 1 << 3,
}

export class PublicFieldImpl {
  public readonly coord: CoordImpl;
  public readonly index: ContinentIndex;

  #flags: Flags = Flags.Uninit;

  private constructor(coord: CoordImpl) {
    this.coord = coord;
    this.index = coord.toIndex();
  }

  public async load(options?: LoadOptions) {
    if (!(this.#flags & Flags.Loading)) {
      this.#flags |= Flags.Loading;
      try {
        await options?.onBeforeLoad?.();
        const field = await getField(this.coord);
        this.setKind(field.kind);
        await options?.onLoad?.();
      }
      catch (err) {
        this.#flags ^= Flags.Loading;
        throw err;
      }
    }
  }

  private init(kind: PublicFieldKind) {
    if (this.#flags & Flags.Uninit) {
      this.setKind(kind);
      return true;
    }

    return false;
  }

  private setKind(kind: PublicFieldKind) {
    switch (kind) {
      case 'empty': {
        this.#flags = Flags.Empty;
        break;
      }
      case 'city': {
        this.#flags = Flags.City;
        break;
      }
    }
  }

  public isUninit() {
    return this.#flags & Flags.Uninit;
  }

  public isLoading() {
    return this.#flags & Flags.Loading;
  }

  public isEmpty() {
    return this.#flags & Flags.Empty;
  }

  public isCity() {
    return this.#flags & Flags.City;
  }

  public isOutside() {
    return this.coord.isOutside();
  }

  public isXOutside() {
    return this.coord.isXOutside();
  }

  public isYOutside() {
    return this.coord.isYOutside();
  }

  get id() {
    return this.coord.id;
  }

  get flags() {
    return this.#flags;
  }

  get x() {
    return this.coord.x;
  }

  get y() {
    return this.coord.y;
  }

  public static create(coord: CoordImpl) {
    return new PublicFieldImpl(coord);
  }

  public static createBulkInitializer() {
    const isInitializing = new Set<ContinentIndex>();
    tryOnScopeDispose(() => isInitializing.clear());

    return async function(fields: readonly PublicFieldImpl[]) {
      const coords: Coord[] = [];
      for (const field of fields) {
        if (
          field.flags === Flags.Uninit &&
          !isInitializing.has(field.index) &&
          !field.coord.isOutside()
        ) {
          coords.push(field.coord);
          isInitializing.add(field.index);
        }
      }

      let counter = 0;
      if (coords.length > 0) {
        for (const [coord, field] of await getFields(coords)) {
          const impl = fields.find((it) => it.coord.is(coord));
          if (impl) {
            impl.init(field.kind);
            isInitializing.delete(impl.index);
            counter++;
          }
        }
      }

      return counter;
    };
  }
}

interface LoadOptions {
  onBeforeLoad?: () => MaybePromise<void>;
  onLoad?: () => MaybePromise<void>;
}
