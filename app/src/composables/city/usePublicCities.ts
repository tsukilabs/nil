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
    return (keysRef.value ?? []).map((key) => CoordImpl.fromContinentKey(key));
  });

  const { state, loading, load } = asyncRef<readonly PublicCityImpl[]>([], async () => {
    const cities = await PublicCityImpl.bulkLoad(coords.value);
    return cities.sort((a, b) => compare(a.name, b.name));
  });

  watch(coords, load);

  return {
    cities: state,
    loading,
    load,
  };
}
