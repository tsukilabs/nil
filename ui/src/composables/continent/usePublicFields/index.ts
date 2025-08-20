// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { tryOnScopeDispose } from '@vueuse/core';
import { shallowRef, type ShallowRef, triggerRef } from 'vue';
import type { CoordImpl } from '@/core/model/continent/coord';
import { PublicFieldImpl } from '@/core/model/continent/public-field';

export type Fields = Readonly<ShallowRef<readonly PublicFieldImpl[]>>;

export function usePublicFields() {
  const fields = shallowRef<PublicFieldImpl[]>([]);
  const cache = new Map<string, PublicFieldImpl>();
  const bulkInit = PublicFieldImpl.createBulkInitializer();
  const triggerFields = () => triggerRef(fields);

  function setCoords(coords: CoordImpl[]) {
    const values: PublicFieldImpl[] = [];
    for (const coord of coords) {
      let field = cache.get(coord.id);
      if (!field) {
        field = PublicFieldImpl.create(coord);
        cache.set(coord.id, field);
      }

      values.push(field);
    }

    initFields(values).err();
    fields.value = values;
  }

  async function initFields(values: PublicFieldImpl[]) {
    if (await bulkInit(values) > 0) {
      triggerFields();
    }
  }

  async function loadCoord(coord: Coord) {
    const field = fields.value.find((it) => it.coord.is(coord));
    if (field && !field.isLoading()) {
      // When loading begins, `PublicFieldImpl` modifies its flags to indicate that an update is taking place.
      // Since Vue relies on these flags to detect changes (due to the use of `key` in the grid loop),
      // we need to trigger the `fields` ref both before and after the update is completed.
      //
      // If the ref were only triggered at the end, Vue would not update the grid because the value
      // of the flag would most likely have already reverted to its initial state by that time.
      await field.load({
        onBeforeLoad: triggerFields,
        onLoad: triggerFields,
      });
    }
  }

  tryOnScopeDispose(() => {
    fields.value = [];
    cache.clear();
  });

  return {
    fields: fields as Fields,
    setCoords,
    loadCoord,
  };
}
