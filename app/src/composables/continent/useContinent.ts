// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { nextMultipleOf } from '@/lib/math';
import { ListenerSet } from '@/lib/listener-set';
import { tryOnScopeDispose } from '@vueuse/core';
import { CoordImpl } from '@/core/model/continent/coord';
import { PublicFieldImpl } from '@/core/model/continent/public-field';
import { computed, ref, type Ref, shallowRef, triggerRef } from 'vue';

export function useContinent(options: {
  width: Readonly<Ref<number>>;
  height: Readonly<Ref<number>>;
  cellSize: Readonly<Ref<number>>;
}) {
  const center = ref(CoordImpl.splat(0));
  const fields = shallowRef<PublicFieldImpl[]>([]);
  const cache = new Map<string, PublicFieldImpl>();
  const bulkInit = PublicFieldImpl.createBulkInitializer();
  const triggerFields = () => triggerRef(fields);

  const { width, height, cellSize } = options;

  const maxCols = computed(() => {
    return nextMultipleOf(Math.ceil(width.value / cellSize.value), 2) + 1;
  });

  const maxRows = computed(() => {
    return nextMultipleOf(Math.ceil(height.value / cellSize.value), 2) + 1;
  });

  const cols = computed(() => {
    const values: [number, Option<number>][] = [];
    for (let col = 0; col < maxCols.value; col++) {
      const field = fields.value.at(col);
      if (field && !field.isXOutside()) {
        values.push([col, field.x]);
      }
      else {
        values.push([col, null]);
      }
    }

    return values;
  });

  const rows = computed(() => {
    const values: [number, Option<number>][] = [];
    for (let row = 0; row < maxRows.value; row++) {
      const field = fields.value.at(row * maxCols.value);
      if (field && !field.isYOutside()) {
        values.push([row, field.y]);
      }
      else {
        values.push([row, null]);
      }
    }

    return values;
  });

  const listener = new ListenerSet();
  listener.event
    .onPublicCityUpdated(({ coord }) => loadCoord(coord));

  tryOnScopeDispose(() => {
    fields.value = [];
    cache.clear();
    listener.dispose();
  });

  function updateCoordsWithin() {
    const xMin = center.value.x - Math.trunc(maxCols.value / 2);
    const xMax = center.value.x + Math.trunc(maxCols.value / 2);
    const yMin = center.value.y - Math.trunc(maxRows.value / 2);
    const yMax = center.value.y + Math.trunc(maxRows.value / 2);

    const newFields: PublicFieldImpl[] = [];

    for (let x = xMin; x <= xMax; x++) {
      for (let y = yMin; y <= yMax; y++) {
        const coord = CoordImpl.create({ x, y });
        let field = cache.get(coord.id);
        if (!field) {
          field = PublicFieldImpl.create(coord);
          cache.set(coord.id, field);
        }

        newFields.push(field);
      }
    }

    newFields.sort((a, b) => {
      if (a.coord.y === b.coord.y) {
        return a.coord.x - b.coord.x;
      }
      else {
        return b.coord.y - a.coord.y;
      }
    });

    fields.value = newFields;

    initFields(newFields).err();
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
      // Since Vue relies on these flags to detect changes (due to the use of `key` in the continent grid loop),
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

  return {
    center,
    fields: fields as Readonly<typeof fields>,
    cols,
    rows,
    maxCols,
    maxRows,
    updateCoordsWithin,
  };
}
