// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { MaybeNilRef } from '@tb-dev/vue';
import { computed, type Ref, toRef } from 'vue';
import type { BuildingImpl } from '@/core/model/infrastructure/building/abstract';

export function useBuildingLevel(building: MaybeNilRef<BuildingImpl>) {
  const buildingRef = toRef(building) as Ref<Option<BuildingImpl>>;
  return computed(() => {
    const current = buildingRef.value?.level ?? 0;
    const min = buildingRef.value?.minLevel ?? 0;
    const max = buildingRef.value?.maxLevel ?? 0;
    return {
      current,
      next: Math.min(current + 1, max),
      previous: Math.max(current - 1, min),
      min,
      max,
      isMin: current <= min,
      isMax: current >= max,
    };
  });
}
