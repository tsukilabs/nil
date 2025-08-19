// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { computed, toRef } from 'vue';
import type { RouteLocationAsRelative } from 'vue-router';

export function useCityOwnerSceneLink(owner: MaybeNilRef<CityOwner>) {
  const ownerRef = toRef(owner);
  return computed<Option<RouteLocationAsRelative>>(() => {
    if (ownerRef.value) {
      const kind = ownerRef.value.kind;
      return {
        name: `profile-${kind}` satisfies ProfileScene,
        params: { id: String(ownerRef.value.id) },
      };
    }

    return null;
  });
}
