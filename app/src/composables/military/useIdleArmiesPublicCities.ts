// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { computed, ref } from "vue";
import { compare } from "@/lib/intl";
import { asyncComputed } from "@tb-dev/vue";
import { PublicCityImpl } from "@/core/model/city/public-city";
import {
  useIdleArmiesCoords,
  type UseIdleArmiesCoordsOptions,
} from "@/composables/military/useIdleArmiesCoords";

type UseIdleArmiesPublicCitiesOptions = UseIdleArmiesCoordsOptions;

export function useIdleArmiesPublicCities(options: UseIdleArmiesPublicCitiesOptions = {}) {
  const {
    coords,
    key,
    loading: isLoadingCoords,
    loadCoords,
  } = useIdleArmiesCoords(options);

  const isLoadingCities = ref(false);
  const cities = asyncComputed<readonly PublicCityImpl[]>([], async () => {
    return PublicCityImpl.bulkLoad(coords.value).then((it) => {
      return it.toSorted((a, b) => compare(a.name, b.name));
    });
  }, {
    evaluating: isLoadingCities,
  });

  const loading = computed(() => {
    return isLoadingCoords.value || isLoadingCities.value;
  });

  return {
    cities,
    key,
    loading,
    loadCoords,
  };
}
