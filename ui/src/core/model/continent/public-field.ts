// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { getField, getFields } from '@/commands';
import { tryOnScopeDispose } from '@vueuse/core';
import type { CoordImpl } from '@/core/model/coord';
import type { MaybePromise, Option } from '@tb-dev/utils';
import { PublicVillageImpl } from '@/core/model/village/public';

enum PublicFieldFlags {
  Uninit = 1,
  Loading = 1 << 1,
  Empty = 1 << 2,
  Village = 1 << 3,
}

export class PublicFieldImpl {
  public readonly coord: CoordImpl;
  #flags: PublicFieldFlags = PublicFieldFlags.Uninit;
  #village: Option<PublicVillageImpl>;

  private constructor(coord: CoordImpl) {
    this.coord = coord;
  }

  public async load(options?: LoadOptions) {
    if (!(this.#flags & PublicFieldFlags.Loading)) {
      this.#flags |= PublicFieldFlags.Loading;
      try {
        await options?.onBeforeLoad?.();
        this.set(await getField(this.coord));
        await options?.onLoad?.();
      } catch (err) {
        this.#flags ^= PublicFieldFlags.Loading;
        throw err;
      }
    }
  }

  private init(field: PublicField) {
    if (this.#flags & PublicFieldFlags.Uninit) {
      this.set(field);
      return true;
    }

    return false;
  }

  private set(field: PublicField) {
    switch (field.kind) {
      case 'empty': {
        this.#flags = PublicFieldFlags.Empty;
        this.#village = null;
        break;
      }
      case 'village': {
        this.#flags = PublicFieldFlags.Village;
        this.#village = PublicVillageImpl.create(field.village);
        break;
      }
    }
  }

  public isUninit() {
    return this.#flags & PublicFieldFlags.Uninit;
  }

  public isLoading() {
    return this.#flags & PublicFieldFlags.Loading;
  }

  public isEmpty() {
    return this.#flags & PublicFieldFlags.Empty;
  }

  public isVillage() {
    return this.#flags & PublicFieldFlags.Village;
  }

  public isOutside(size: number) {
    return this.coord.isOutside(size);
  }

  public isXOutside(size: number) {
    return this.coord.isXOutside(size);
  }

  public isYOutside(size: number) {
    return this.coord.isYOutside(size);
  }

  get id() {
    return this.coord.id;
  }

  get flags() {
    return this.#flags;
  }

  get village() {
    return this.#village;
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

  public static createBulkInitializer(continentSize: number) {
    const isInitializing = new Set<string>();
    tryOnScopeDispose(() => isInitializing.clear());

    return async function (fields: readonly PublicFieldImpl[]) {
      const coords: Coord[] = [];
      for (const field of fields) {
        if (
          field.flags === PublicFieldFlags.Uninit &&
          !isInitializing.has(field.id) &&
          !field.coord.isOutside(continentSize)
        ) {
          coords.push(field.coord);
          isInitializing.add(field.id);
        }
      }

      let counter = 0;
      if (coords.length > 0) {
        for (const [coord, field] of await getFields(coords)) {
          const impl = fields.find((it) => it.coord.is(coord));
          if (impl) {
            impl.init(field);
            isInitializing.delete(impl.id);
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
