// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { computed } from 'vue';
import type { MaybeNilRef } from '@tb-dev/vue';
import { toCityRef } from '@/composables/util/toRef';
import type { CityImpl } from '@/core/model/city/city';

export function useInfrastructure(city?: MaybeNilRef<CityImpl>) {
  const cityRef = toCityRef(city);
  return computed(() => cityRef.value?.infrastructure);
}
