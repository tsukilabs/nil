// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { computed, type MaybeRefOrGetter, toRef } from 'vue';
import { ArmyPersonnelImpl } from '@/core/model/military/army-personnel';

export function foldArmyPersonnel(armies: MaybeRefOrGetter<readonly Army[]>) {
  const armiesRef = toRef(armies);
  return computed(() => {
    const initial = ArmyPersonnelImpl.createEmptyRaw() as WritableArmyPersonnel;
    const personnel = armiesRef.value.reduce((acc, curr) => {
      acc.archer.size += curr.personnel.archer.size;
      acc.axeman.size += curr.personnel.axeman.size;
      acc.heavyCavalry.size += curr.personnel.heavyCavalry.size;
      acc.lightCavalry.size += curr.personnel.lightCavalry.size;
      acc.pikeman.size += curr.personnel.pikeman.size;
      acc.swordsman.size += curr.personnel.swordsman.size;
      return acc;
    }, initial);

    return ArmyPersonnelImpl.create(personnel);
  });
}
