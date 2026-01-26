// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { computed, type Ref, toRef } from 'vue';
import { useBuildingLevel } from '@/composables/infrastructure/useBuildingLevel';
import type { StorageImpl } from '@/core/model/infrastructure/building/abstract';

export function useStorageStats(storage: MaybeNilRef<StorageImpl>) {
  const storageRef = toRef(storage) as Ref<Option<StorageImpl>>;
  const level = useBuildingLevel(storageRef);

  const stats = computed(() => {
    return {
      current: storageRef.value?.getStorageStats(),
      next: storageRef.value?.getStorageStatsBy(level.value.next),
    };
  });

  return { level, stats };
}
