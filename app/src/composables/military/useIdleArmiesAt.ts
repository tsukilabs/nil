// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { computed } from 'vue';
import type { MaybeNilRef } from '@tb-dev/vue';
import { toContinentKeyRef } from '@/composables/toRef';
import type { ArmyImpl } from '@/core/model/military/army';

export function useIdleArmiesAt(key?: MaybeNilRef<ContinentKey>) {
  const keyRef = toContinentKeyRef(key);
  const { military } = NIL.military.refs();
  return computed<readonly ArmyImpl[]>(() => {
    if (keyRef.value && military.value) {
      return military.value.getIdleArmiesAt(keyRef.value);
    }

    return [];
  });
}
