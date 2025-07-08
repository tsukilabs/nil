// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { getFields } from '@/commands';
import type { Option } from '@tb-dev/utils';
import { tryOnScopeDispose } from '@vueuse/core';
import type { CoordImpl } from '@/core/model/coord';
import { PublicVillageImpl } from '@/core/model/village/public';

const enum Kind {
  Uninit = 0,
  Empty = 1,
  Village = 2,
}

export class PublicFieldImpl {
  public readonly coord: CoordImpl;
  #kind: Kind = Kind.Uninit;
  #village: Option<PublicVillageImpl>;

  private constructor(coord: CoordImpl) {
    this.coord = coord;
  }

  public init(field: PublicField) {
    if (this.#kind !== Kind.Uninit) {
      return false;
    }

    if (field.kind === 'village') {
      this.#kind = Kind.Village;
      this.#village = PublicVillageImpl.create(field.village);
    }

    return true;
  }

  public isUninit() {
    return this.#kind === Kind.Uninit;
  }

  public isEmpty() {
    return this.#kind === Kind.Empty;
  }

  public isVillage() {
    return this.#kind === Kind.Village;
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

  get kind() {
    return this.#kind;
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
          field.kind === Kind.Uninit &&
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
