// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { computed } from 'vue';
import type { MaybeNilRef } from '@tb-dev/vue';
import { toVillageRef } from '@/composables/util/toRef';
import type { VillageImpl } from '@/core/model/village/village';

export function useAcademy(village?: MaybeNilRef<VillageImpl>) {
  const villageRef = toVillageRef(village);
  return computed(() => villageRef.value?.infrastructure.academy);
}

export function useFarm(village?: MaybeNilRef<VillageImpl>) {
  const villageRef = toVillageRef(village);
  return computed(() => villageRef.value?.infrastructure.farm);
}

export function useIronMine(village?: MaybeNilRef<VillageImpl>) {
  const villageRef = toVillageRef(village);
  return computed(() => villageRef.value?.infrastructure.ironMine);
}

export function usePrefecture(village?: MaybeNilRef<VillageImpl>) {
  const villageRef = toVillageRef(village);
  return computed(() => villageRef.value?.infrastructure.prefecture);
}

export function useQuarry(village?: MaybeNilRef<VillageImpl>) {
  const villageRef = toVillageRef(village);
  return computed(() => villageRef.value?.infrastructure.quarry);
}

export function useSawmill(village?: MaybeNilRef<VillageImpl>) {
  const villageRef = toVillageRef(village);
  return computed(() => villageRef.value?.infrastructure.sawmill);
}

export function useSilo(village?: MaybeNilRef<VillageImpl>) {
  const villageRef = toVillageRef(village);
  return computed(() => villageRef.value?.infrastructure.silo);
}

export function useStable(village?: MaybeNilRef<VillageImpl>) {
  const villageRef = toVillageRef(village);
  return computed(() => villageRef.value?.infrastructure.stable);
}

export function useWall(village?: MaybeNilRef<VillageImpl>) {
  const villageRef = toVillageRef(village);
  return computed(() => villageRef.value?.infrastructure.wall);
}

export function useWarehouse(village?: MaybeNilRef<VillageImpl>) {
  const villageRef = toVillageRef(village);
  return computed(() => villageRef.value?.infrastructure.warehouse);
}
