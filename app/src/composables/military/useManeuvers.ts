// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { computed } from 'vue';
import { toContinentKeyRef } from '@/composables/util/toRef';

export function useManeuvers(key?: MaybeNilRef<ContinentKey>) {
  const keyRef = toContinentKeyRef(key);
  const { military } = NIL.military.refs();

  return computed(() => {
    if (military.value && keyRef.value) {
      return military.value.getManeuversAt(keyRef.value);
    }
    else {
      return [];
    }
  });
}
