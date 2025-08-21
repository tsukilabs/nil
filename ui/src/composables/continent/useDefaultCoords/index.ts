// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { computed } from 'vue';
import { useRouteQuery } from '@vueuse/router';
import { CoordImpl, isOutside } from '@/core/model/continent/coord';

export function useQueryCoords() {
  const { coord: currentCoord } = NIL.city.refs();
  const qX = useRouteQuery('x', null, { transform });
  const qY = useRouteQuery('y', null, { transform });

  return computed(() => {
    const currX = currentCoord.value?.x;
    const currY = currentCoord.value?.y;

    if (isValid(qX.value) && isValid(qY.value)) {
      return CoordImpl.create({ x: qX.value, y: qY.value });
    }
    else if (isValid(currX) && isValid(currY)) {
      return CoordImpl.create({ x: currX, y: currY });
    }

    return null;
  });
}

function transform(value: string) {
  const coord = Number.parseInt(value);
  return isValid(coord) ? coord : null;
}

function isValid(coord: Option<number>): coord is number {
  return typeof coord === 'number' && !isOutside(coord);
}
