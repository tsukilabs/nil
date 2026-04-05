// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { computed, type MaybeRefOrGetter, toRef } from 'vue';
import type { ArmyPersonnel } from '@/types/core/military/army';
import { ArmyPersonnelImpl } from '@/core/model/military/army-personnel';

export function useArmyPersonnelSize(personnel: MaybeRefOrGetter<ArmyPersonnel>) {
  const personnelRef = toRef(personnel);
  return computed(() => ArmyPersonnelImpl.getSize(personnelRef.value));
}
