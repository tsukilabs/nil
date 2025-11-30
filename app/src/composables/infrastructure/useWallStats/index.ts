// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { computed, type Ref, toRef } from 'vue';
import type { WallImpl } from '@/core/model/infrastructure/building/wall';
import { useBuildingLevel } from '@/composables/infrastructure/useBuildingLevel';

export function useWallStats(wall: MaybeNilRef<WallImpl>) {
  const wallRef = toRef(wall) as Ref<Option<WallImpl>>;
  const level = useBuildingLevel(wallRef);

  const stats = computed(() => {
    return {
      current: wallRef.value?.getWallStats(),
      next: wallRef.value?.getWallStatsBy(level.value.next),
    };
  });

  return { level, stats };
}
