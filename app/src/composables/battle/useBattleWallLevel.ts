// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { computed, type MaybeRefOrGetter, toRef } from 'vue';
import type { BattleResultImpl } from '@/core/model/battle-result';

export function useBattleWallLevel(result: MaybeRefOrGetter<BattleResultImpl>) {
  const resultRef = toRef(result);
  return computed(() => {
    const original = resultRef.value.wallLevel;
    const current = resultRef.value.resolveWallLevel();
    return {
      original,
      current,
      didChange: current !== original,
    };
  });
}
