// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { computed } from "vue";
import type { MaybeNilRef } from "@tb-dev/vue";
import { toContinentKeyRef } from "@/composables/toRef";
import type { ContinentKey } from "@/types/core/continent";

export function useManeuversAt(key?: MaybeNilRef<ContinentKey>) {
  const keyRef = toContinentKeyRef(key);
  const { military } = NIL.military.refs();

  return computed(() => {
    if (military.value && keyRef.value) {
      return military.value.getManeuversAt(keyRef.value);
    }
    else {
      return [];
    }
  });
}
