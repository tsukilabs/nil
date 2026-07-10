// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { readonly } from "vue";
import * as commands from "@/commands";
import { asyncRef } from "@tb-dev/vue";
import { CoordImpl } from "@/core/model/continent/coord";
import { useRandomKey } from "@/composables/useRandomKey";

export interface UseIdleArmiesCoordsOptions {
  includeOwnCoords?: boolean;
}

export function useIdleArmiesCoords(options: UseIdleArmiesCoordsOptions = {}) {
  options.includeOwnCoords ??= true;

  const { key, updateRandomKey } = useRandomKey();
  const { state, loading, load } = asyncRef<readonly CoordImpl[]>([], async () => {
    const coords = await commands.getIdleArmiesCoords().then((it) => {
      return it.map((coord) => CoordImpl.create(coord));
    });

    if (options.includeOwnCoords) {
      const playerCoords = NIL.player.getCoords();
      for (const coord of playerCoords) {
        if (coords.every((it) => !it.is(coord))) {
          coords.push(coord);
        }
      }
    }

    return coords;
  });

  return {
    coords: state as Readonly<typeof state>,
    key: readonly(key),
    loading,
    loadCoords: () => load().then(updateRandomKey),
  };
}
