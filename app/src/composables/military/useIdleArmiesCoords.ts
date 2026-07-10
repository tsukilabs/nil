// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { readonly } from "vue";
import * as commands from "@/commands";
import { asyncRef } from "@tb-dev/vue";
import { CoordImpl } from "@/core/model/continent/coord";
import { useRandomKey } from "@/composables/useRandomKey";

export interface UseIdleArmiesCoordsOptions {
  includeCurrent?: boolean;
}

export function useIdleArmiesCoords(options: UseIdleArmiesCoordsOptions = {}) {
  options.includeCurrent ??= true;

  const { coord: currentCoord } = NIL.city.refs();
  const { key, updateRandomKey } = useRandomKey();

  const { state, loading, load } = asyncRef<readonly CoordImpl[]>([], async () => {
    const coords = await commands.getIdleArmiesCoords();

    if (options.includeCurrent) {
      const current = currentCoord.value;
      if (current && coords.every((it) => !current.is(it))) {
        coords.push(current);
      }
    }

    return coords.map((coord) => CoordImpl.create(coord));
  });

  return {
    coords: state as Readonly<typeof state>,
    key: readonly(key),
    loading,
    loadCoords: () => load().then(updateRandomKey),
  };
}
