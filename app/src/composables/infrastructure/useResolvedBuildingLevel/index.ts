// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { computed, type MaybeRefOrGetter, type Ref, toRef } from 'vue';
import { usePrefecture } from '@/composables/infrastructure/useBuilding';
import { useBuildingLevel } from '@/composables/infrastructure/useBuildingLevel';
import type { BuildingImpl } from '@/core/model/infrastructure/building/abstract';

export function useResolvedBuildingLevel(building: MaybeRefOrGetter<BuildingImpl>) {
  const prefecture = usePrefecture();
  const buildingRef = toRef(building) as Ref<BuildingImpl>;
  const level = useBuildingLevel(buildingRef);

  return computed(() => {
    const current = prefecture.value?.resolveBuildingLevel(buildingRef.value) ?? 0;
    return {
      min: level.value.min,
      max: level.value.max,
      current,
      next: current + 1,
      previous: current - 1,
    };
  });
}
