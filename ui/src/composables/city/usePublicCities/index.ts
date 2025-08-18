// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { compare } from '@/lib/intl';
import { asyncRef } from '@tb-dev/vue';
import { computed, toRef, watch } from 'vue';
import { CoordImpl } from '@/core/model/continent/coord';
import { getPublicCitiesBy, getPublicCity } from '@/commands';
import { PublicCityImpl } from '@/core/model/city/public-city';

export function usePublicCities(keys: MaybeNilRef<readonly ContinentKey[]>) {
  const keysRef = toRef(keys);
  const coords = computed(() => {
    if (keysRef.value && keysRef.value.length > 0) {
      return keysRef.value.map((key) => CoordImpl.fromKey(key));
    }

    return null;
  });

  const { state, isLoading, execute } = asyncRef([], async () => {
    const cities: PublicCityImpl[] = [];
    if (coords.value) {
      if (coords.value.length === 1) {
        const city = await getPublicCity(coords.value[0]);
        cities.push(PublicCityImpl.create(city));
      }
      else {
        for (const city of await getPublicCitiesBy(coords.value)) {
          cities.push(PublicCityImpl.create(city));
        }
      }
    }

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
