// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { computed, toRef } from "vue";
import type { Option } from "@tb-dev/utils";
import type { MaybeNilRef } from "@tb-dev/vue";
import type { PublicCity } from "@tsukilabs/nil-bindings";
import type { ProfileScene } from "@/types/scene/game";
import { CoordImpl } from "@/core/model/continent/coord";
import type { RouteLocationAsRelative } from "vue-router";

export function useCityProfileSceneLink(city: MaybeNilRef<PublicCity>) {
  const cityRef = toRef(city);
  return computed<Option<RouteLocationAsRelative>>(() => {
    const coord = cityRef.value?.coord;
    const ckey = coord ? CoordImpl.toContinentIndexString(coord) : null;
    if (ckey) {
      return {
        name: "profile-city" satisfies ProfileScene,
        params: { ckey },
      };
    }

    return null;
  });
}
