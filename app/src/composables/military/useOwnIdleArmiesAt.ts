// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { computed } from "vue";
import type { MaybeNilRef } from "@tb-dev/vue";
import type { ArmyImpl } from "@/core/model/military/army";
import type { ContinentKey } from "@/types/core/continent";
import { useIdleArmiesAt } from "@/composables/military/useIdleArmiesAt";

export function useOwnIdleArmiesAt(key?: MaybeNilRef<ContinentKey>) {
  const armies = useIdleArmiesAt(key);
  const { player } = NIL.player.refs();

  return computed<readonly ArmyImpl[]>(() => {
    return armies.value.filter((army) => {
      return army.owner.kind === "player" && army.owner.id === player.value?.id;
    });
  });
}
