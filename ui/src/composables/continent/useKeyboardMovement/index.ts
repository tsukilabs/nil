// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { Fn } from '@tb-dev/utils';
import { onKeyDown } from '@vueuse/core';
import { type Continent, Coord as WasmCoord } from 'nil-continent';

export function useKeyboardMovement(continent: Continent, render: Fn) {
  const { continentSize } = NIL.world.refs();

  onKeyDown('ArrowUp', move('up'), { dedupe: false });
  onKeyDown('ArrowDown', move('down'), { dedupe: false });
  onKeyDown('ArrowLeft', move('left'), { dedupe: false });
  onKeyDown('ArrowRight', move('right'), { dedupe: false });

  function move(dir: 'up' | 'down' | 'left' | 'right') {
    return function(e: KeyboardEvent) {
      const center = continent.center();
      let x = center.x();
      let y = center.y();

      let delta = 1;
      if (e.ctrlKey) delta = 5;
      if (e.shiftKey) delta = 10;
      if (e.ctrlKey && e.shiftKey) delta = 25;

      if (dir === 'up' && y + delta <= continentSize.value) {
        y += delta;
      }
      else if (dir === 'down' && y - delta >= 0) {
        y -= delta;
      }
      else if (dir === 'left' && x - delta >= 0) {
        x -= delta;
      }
      else if (dir === 'right' && x + delta <= continentSize.value) {
        x += delta;
      }

      continent.set_center(new WasmCoord(x, y));

      render();
    };
  }
}
