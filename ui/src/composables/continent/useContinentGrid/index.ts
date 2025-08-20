// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { computed, ref } from 'vue';
import type { Fields } from '../usePublicFields';

export function useContinentGrid(fields: Fields) {
  const gridCols = ref(0);
  const gridRows = ref(0);

  const cols = computed(() => {
    const values: [number, Option<number>][] = [];
    for (let col = 0; col < gridCols.value; col++) {
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
    for (let row = 0; row < gridRows.value; row++) {
      const field = fields.value.at(row * gridCols.value);
      if (field && !field.isYOutside()) {
        values.push([row, field.y]);
      }
      else {
        values.push([row, null]);
      }
    }

    return values;
  });

  return {
    gridCols,
    gridRows,
    cols,
    rows,
  };
}
