// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { BuildingImpl } from '@/core/model/buildings/abstract';
import { computed, type MaybeRefOrGetter, type Ref, toRef } from 'vue';
import type { PrefectureImpl } from '@/core/model/buildings/prefecture';

export function useBuildingLevel(
  prefecture: MaybeRefOrGetter<PrefectureImpl>,
  building: MaybeRefOrGetter<BuildingImpl>
) {
  const prefectureRef = toRef(prefecture);
  const buildingRef = toRef(building) as Ref<BuildingImpl>;

  return computed(() => {
    const min = buildingRef.value.minLevel;
    const max = buildingRef.value.maxLevel;
    const current = prefectureRef.value.resolveBuildingLevel(buildingRef.value);

    return {
      min,
      max,
      current,
      next: current + 1,
      previous: current - 1,
    };
  });
}
