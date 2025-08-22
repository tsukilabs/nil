// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { MaybeNilRef } from '@tb-dev/vue';
import { computed, type Ref, toRef } from 'vue';
import type { MineImpl } from '@/core/model/infrastructure/building/abstract';
import { useBuildingLevel } from '@/composables/infrastructure/useBuildingLevel';

export function useMineStats(mine: MaybeNilRef<MineImpl>) {
  const { city } = NIL.city.refs();
  const mineRef = toRef(mine) as Ref<Option<MineImpl>>;
  const level = useBuildingLevel(mineRef);

  const stats = computed(() => {
    return {
      current: mineRef.value?.getMineStats(),
      next: mineRef.value?.getMineStatsBy(level.value.next),
    };
  });

  const base = computed(() => {
    return {
      current: stats.value.current?.production ?? 0,
      next: stats.value.next?.production ?? 0,
    };
  });

  const stabilityLoss = computed(() => {
    const stability = city.value?.stability ?? 1;
    return {
      current: base.value.current - base.value.current * stability,
      next: base.value.next - base.value.next * stability,
    };
  });

  const actual = computed(() => {
    const current = Math.ceil(base.value.current - stabilityLoss.value.current);
    const next = Math.ceil(base.value.next - stabilityLoss.value.next);
    return {
      current: Math.max(current, 0),
      next: Math.max(next, 0),
    };
  });

  return { level, stats, base, stabilityLoss, actual };
}
