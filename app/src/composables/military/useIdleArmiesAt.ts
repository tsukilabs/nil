// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import * as commands from "@/commands";
import { ArmyImpl } from "@/core/model/military/army";
import { toContinentKeyRef } from "@/composables/toRef";
import { CoordImpl } from "@/core/model/continent/coord";
import type { ContinentKey } from "@/types/core/continent";
import { asyncComputed, type MaybeNilRef } from "@tb-dev/vue";

export function useIdleArmiesAt(key?: MaybeNilRef<ContinentKey>) {
  const keyRef = toContinentKeyRef(key);
  const { player } = NIL.player.refs();
  const { military } = NIL.military.refs();

  return asyncComputed<readonly ArmyImpl[]>([], async () => {
    if (keyRef.value) {
      const coord = CoordImpl.fromContinentKey(keyRef.value);
      if (player.value?.owns(coord)) {
        return military.value?.getIdleArmiesAt(coord) ?? [];
      }
      else {
        const armies = await commands.getIdleArmiesAt(coord);
        return armies.map((army) => ArmyImpl.create(army));
      }
    }

    return [];
  });
}
