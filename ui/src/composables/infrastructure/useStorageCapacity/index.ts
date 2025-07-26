// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { computed, type Ref, toRef } from 'vue';
import type { StorageImpl } from '@/core/model/infrastructure/building/abstract';
import { useBuildingLevel } from '@/composables/infrastructure/useBuildingLevel';

export function useStorageCapacity(storage: MaybeNilRef<StorageImpl>) {
  const storageRef = toRef(storage) as Ref<Option<StorageImpl>>;
  const level = useBuildingLevel(storageRef);

  const capacity = computed(() => {
    return {
      current: storageRef.value?.getCapacity() ?? 0,
      next: storageRef.value?.getCapacityBy(level.value.next) ?? 0,
    };
  });

  return { level, capacity };
}
