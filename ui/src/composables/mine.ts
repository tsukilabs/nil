// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { Option } from '@tb-dev/utils';
import type { MaybeNilRef } from '@tb-dev/vue';
import { computed, type Ref, toRef } from 'vue';
import type { MineImpl } from '@/core/model/buildings/abstract';

export function useMineLevel(mine: MaybeNilRef<MineImpl>) {
  const mineRef = toRef(mine) as Ref<Option<MineImpl>>;
  return computed(() => {
    const current = mineRef.value?.level ?? 0;
    const min = mineRef.value?.minLevel ?? 0;
    const max = mineRef.value?.maxLevel ?? 0;
    return {
      current,
      next: Math.min(current + 1, max),
      previous: Math.max(current - 1, min),
      min,
      max,
      isMin: current <= min,
      isMax: current >= max,
    };
  });
}

export function useMineProduction(mine: MaybeNilRef<MineImpl>) {
  const { village } = NIL.village.refs();
  const mineRef = toRef(mine) as Ref<Option<MineImpl>>;
  const level = useMineLevel(mineRef);

  const base = computed(() => {
    return {
      current: mineRef.value?.getProduction() ?? 0,
      next: mineRef.value?.getProductionBy(level.value.next) ?? 0,
    };
  });

  const stabilityLoss = computed(() => {
    const stability = village.value?.stability ?? 1;
    return {
      current: base.value.current - base.value.current * stability,
      next: base.value.next - base.value.next * stability,
    };
  });

  const actual = computed(() => {
    const current = Math.ceil(base.value.current - stabilityLoss.value.current);
    const next = Math.ceil(base.value.next - stabilityLoss.value.next);
    return { current: Math.max(current, 0), next: Math.max(next, 0) };
  });

  return { level, base, stabilityLoss, actual };
}
