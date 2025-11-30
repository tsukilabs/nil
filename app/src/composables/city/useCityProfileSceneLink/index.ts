// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { computed, toRef } from 'vue';
import type { RouteLocationAsRelative } from 'vue-router';
import type { PublicCityImpl } from '@/core/model/city/public-city';

export function useCityProfileSceneLink(city: MaybeNilRef<PublicCityImpl>) {
  const cityRef = toRef(city);
  return computed<Option<RouteLocationAsRelative>>(() => {
    const ckey = cityRef.value?.coord.toIndexString();
    if (ckey) {
      return {
        name: 'profile-city' satisfies ProfileScene,
        params: { ckey },
      };
    }

    return null;
  });
}
