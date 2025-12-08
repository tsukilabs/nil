// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { ref } from 'vue';
import { CoordImpl } from '@/core/model/continent/coord';
import { isValid, transform } from '@/composables/continent/useQueryCoord';
import {
  QUERY_WAR_ROOM_DEST_X,
  QUERY_WAR_ROOM_DEST_Y,
  QUERY_WAR_ROOM_ORIGIN_X,
  QUERY_WAR_ROOM_ORIGIN_Y,
} from '@/router/game/war-room';

export function useWarRoomCoords() {
  const url = new URL(window.location.href);
  const originX = transform(url.searchParams.get(QUERY_WAR_ROOM_ORIGIN_X));
  const originY = transform(url.searchParams.get(QUERY_WAR_ROOM_ORIGIN_Y));
  const destX = transform(url.searchParams.get(QUERY_WAR_ROOM_DEST_X));
  const destY = transform(url.searchParams.get(QUERY_WAR_ROOM_DEST_Y));

  const origin = ref(getDefaultOrigin());
  if (isValid(originX) && isValid(originY)) {
    origin.value = CoordImpl.create({ x: originX, y: originY });
  }

  const destination = ref(CoordImpl.splat(0));
  if (isValid(destX) && isValid(destY)) {
    destination.value = CoordImpl.create({ x: destX, y: destY });
  }

  return {
    origin,
    destination,
  };
}

function getDefaultOrigin() {
  return NIL.city.getCoord() ??
    NIL.player.getCoords().at(0) ??
    CoordImpl.splat(0);
}
