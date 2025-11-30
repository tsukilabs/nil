// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { compare } from '@/lib/intl';
import { asyncRef } from '@tb-dev/vue';
import { computed, toRef, watch } from 'vue';
import { CoordImpl } from '@/core/model/continent/coord';
import { PublicCityImpl } from '@/core/model/city/public-city';

export function usePublicCities(keys: MaybeNilRef<readonly ContinentKey[]>) {
  const keysRef = toRef(keys);
  const coords = computed(() => {
    return (keysRef.value ?? []).map((key) => CoordImpl.fromKey(key));
  });

  const { state, isLoading, execute } = asyncRef([], async () => {
    const cities: PublicCityImpl[] = [];
    await Promise.all(coords.value.map(async (coord) => {
      cities.push(await PublicCityImpl.load(coord));
    }));

    cities.sort((a, b) => compare(a.name, b.name));

    return cities as readonly PublicCityImpl[];
  });

  watch(coords, execute);

  return {
    cities: state,
    loading: isLoading,
    load: execute,
  };
}
