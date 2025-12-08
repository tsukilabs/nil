// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { shallowRef } from 'vue';
import { useRouteQuery } from '@vueuse/router';
import { CoordImpl, isOutside } from '@/core/model/continent/coord';

export function useQueryCoord() {
  const initialCoord = shallowRef<Option<CoordImpl>>();
  const queryX = useRouteQuery('x', null, { transform });
  const queryY = useRouteQuery('y', null, { transform });

  if (isValid(queryX.value) && isValid(queryY.value)) {
    initialCoord.value = CoordImpl.create({ x: queryX.value, y: queryY.value });
  }
  else {
    const currentCoord = NIL.city.getCoord();
    const currX = currentCoord?.x;
    const currY = currentCoord?.y;

    if (isValid(currX) && isValid(currY)) {
      initialCoord.value = CoordImpl.create({ x: currX, y: currY });
    }
  }

  return { initialCoord, queryX, queryY };
}

export function transform(value: Option<string>) {
  if (value) {
    const coord = Number.parseInt(value);
    if (isValid(coord)) return coord;
  }

  return null;
}

export function isValid(coord: Option<number>): coord is number {
  return typeof coord === 'number' && !isOutside(coord);
}
