// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { Fn } from '@tb-dev/utils';
import { onKeyDown } from '@vueuse/core';
import type { Continent } from 'nil-continent';

export function onKeyboardMovement(continent: Continent, render: Fn) {
  const { continentSize } = NIL.world.refs();

  onKeyDown('ArrowUp', move('up'), { dedupe: false });
  onKeyDown('ArrowDown', move('down'), { dedupe: false });
  onKeyDown('ArrowLeft', move('left'), { dedupe: false });
  onKeyDown('ArrowRight', move('right'), { dedupe: false });

  function move(dir: 'up' | 'down' | 'left' | 'right') {
    return function(e: KeyboardEvent) {
      const center = continent.center();
      const initialX = center.x();
      const initialY = center.y();

      let x = initialX;
      let y = initialY;

      let delta = 1;
      if (e.ctrlKey) delta = 5;
      if (e.shiftKey) delta = 10;
      if (e.ctrlKey && e.shiftKey) delta = 25;

      switch (dir) {
        case 'up': {
          y = Math.min(y + delta, continentSize.value - 1);
          break;
        }
        case 'down': {
          y = Math.max(y - delta, 0);
          break;
        }
        case 'left': {
          x = Math.max(x - delta, 0);
          break;
        }
        case 'right': {
          x = Math.min(x + delta, continentSize.value - 1);
          break;
        }
      }

      if (x !== initialX || y !== initialY) {
        continent.set_center(x, y);
        render();
      }
    };
  }
}
