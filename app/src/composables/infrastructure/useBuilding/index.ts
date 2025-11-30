// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { computed } from 'vue';
import type { MaybeNilRef } from '@tb-dev/vue';
import { toCityRef } from '@/composables/util/toRef';
import type { CityImpl } from '@/core/model/city/city';

export function useAcademy(city?: MaybeNilRef<CityImpl>) {
  const cityRef = toCityRef(city);
  return computed(() => cityRef.value?.infrastructure.academy);
}

export function useFarm(city?: MaybeNilRef<CityImpl>) {
  const cityRef = toCityRef(city);
  return computed(() => cityRef.value?.infrastructure.farm);
}

export function useIronMine(city?: MaybeNilRef<CityImpl>) {
  const cityRef = toCityRef(city);
  return computed(() => cityRef.value?.infrastructure.ironMine);
}

export function usePrefecture(city?: MaybeNilRef<CityImpl>) {
  const cityRef = toCityRef(city);
  return computed(() => cityRef.value?.infrastructure.prefecture);
}

export function useQuarry(city?: MaybeNilRef<CityImpl>) {
  const cityRef = toCityRef(city);
  return computed(() => cityRef.value?.infrastructure.quarry);
}

export function useSawmill(city?: MaybeNilRef<CityImpl>) {
  const cityRef = toCityRef(city);
  return computed(() => cityRef.value?.infrastructure.sawmill);
}

export function useSilo(city?: MaybeNilRef<CityImpl>) {
  const cityRef = toCityRef(city);
  return computed(() => cityRef.value?.infrastructure.silo);
}

export function useStable(city?: MaybeNilRef<CityImpl>) {
  const cityRef = toCityRef(city);
  return computed(() => cityRef.value?.infrastructure.stable);
}

export function useWall(city?: MaybeNilRef<CityImpl>) {
  const cityRef = toCityRef(city);
  return computed(() => cityRef.value?.infrastructure.wall);
}

export function useWarehouse(city?: MaybeNilRef<CityImpl>) {
  const cityRef = toCityRef(city);
  return computed(() => cityRef.value?.infrastructure.warehouse);
}
