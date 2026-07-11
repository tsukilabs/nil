// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { ref, shallowRef } from "vue";
import type { Option } from "@tb-dev/utils";
import { watchDebounced } from "@vueuse/core";
import { useRouteQuery } from "@vueuse/router";
import { CoordImpl } from "@/core/model/continent/coord";
import { isValid, transform } from "@/composables/continent/useQueryCoord";
import {
  QUERY_WAR_ROOM_DEST_X,
  QUERY_WAR_ROOM_DEST_Y,
  QUERY_WAR_ROOM_ORIGIN_X,
  QUERY_WAR_ROOM_ORIGIN_Y,
} from "@/router/game/war-room";

export function useWarRoomCoords() {
  const origin = shallowRef(getDefaultOrigin());
  const destination = ref(CoordImpl.splat(0));

  const originX = useRouteQuery(QUERY_WAR_ROOM_ORIGIN_X, null, { transform });
  const originY = useRouteQuery(QUERY_WAR_ROOM_ORIGIN_Y, null, { transform });
  const destX = useRouteQuery(QUERY_WAR_ROOM_DEST_X, null, { transform });
  const destY = useRouteQuery(QUERY_WAR_ROOM_DEST_Y, null, { transform });

  if (isValid(originX.value) && isValid(originY.value)) {
    origin.value = CoordImpl.create({ x: originX.value, y: originY.value });
  }

  if (isValid(destX.value) && isValid(destY.value)) {
    destination.value = CoordImpl.create({ x: destX.value, y: destY.value });
  }

  watchDebounced([origin, destination], updateRouteQuery, {
    debounce: 200,
    maxWait: 500,
    immediate: true,
  });

  function updateRouteQuery() {
    originX.value = origin.value?.x ?? null;
    originY.value = origin.value?.y ?? null;
    destX.value = destination.value.x;
    destY.value = destination.value.y;
  }

  return {
    origin,
    destination,
  };
}

function getDefaultOrigin(): Option<CoordImpl> {
  return NIL.city.getCoord() ?? NIL.player.getCoords().at(0);
}
