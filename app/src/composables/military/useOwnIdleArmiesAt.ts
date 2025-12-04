// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { computed } from 'vue';
import type { MaybeNilRef } from '@tb-dev/vue';
import type { ArmyImpl } from '@/core/model/military/army';
import { toContinentKeyRef } from '@/composables/util/toRef';

export function useOwnIdleArmiesAt(key?: MaybeNilRef<ContinentKey>) {
  const keyRef = toContinentKeyRef(key);
  const { military } = NIL.military.refs();
  return computed<readonly ArmyImpl[]>(() => {
    if (keyRef.value && military.value) {
      return military.value.getOwnIdleArmiesAt(keyRef.value);
    }

    return [];
  });
}
