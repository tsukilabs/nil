// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { computed, toRef } from "vue";
import type { MaybeNilRef } from "@tb-dev/vue";
import type { Ruler } from "@tsukilabs/nil-bindings";
import type { DeepReadonly } from "es-toolkit/types";

export function useRulersByKind(
  rulers: MaybeNilRef<readonly Ruler[]>,
  options?: UseRulerKindsOptions,
) {
  const rulersRef = toRef(rulers);
  const { id: currentPlayerId } = NIL.player.refs();

  const bots = computed<DeepReadonly<Ruler[]>>(() => {
    return rulersRef.value?.filter((it) => it.kind === "bot") ?? [];
  });

  const players = computed<DeepReadonly<Ruler[]>>(() => {
    const values = rulersRef.value?.filter((it) => {
      return it.kind === "player" &&
        (options?.allowCurrentPlayer ?? it.id !== currentPlayerId.value);
    });

    return values ?? [];
  });

  const precursors = computed<DeepReadonly<Ruler[]>>(() => {
    return rulersRef.value?.filter((it) => it.kind === "precursor") ?? [];
  });

  return {
    bots,
    players,
    precursors,
  };
}

export interface UseRulerKindsOptions {
  allowCurrentPlayer?: boolean;
}
