// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { computed, type MaybeRefOrGetter, toRef } from 'vue';
import { BattleResultImpl } from '@/core/model/battle-result';

export function useBattleLosses(result: MaybeRefOrGetter<BattleResult>) {
  const resultRef = toRef(result);
  return computed(() => {
    const attackerPersonnel = resultRef.value.attackerPersonnel;
    const attackerSurvivingPersonnel = resultRef.value.attackerSurvivingPersonnel;
    const defenderPersonnel = resultRef.value.defenderPersonnel;
    const defenderSurvivingPersonnel = resultRef.value.defenderSurvivingPersonnel;

    const attackerLosses = BattleResultImpl.getLosses(
      attackerPersonnel,
      attackerSurvivingPersonnel,
    );

    const defenderLosses = BattleResultImpl.getLosses(
      defenderPersonnel,
      defenderSurvivingPersonnel,
    );

    return {
      attackerLosses,
      defenderLosses,
    };
  });
}
