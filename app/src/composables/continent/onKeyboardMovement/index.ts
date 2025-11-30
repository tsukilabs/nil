// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { onKeyDown } from '@vueuse/core';

export type Direction = 'up' | 'down' | 'left' | 'right';

type MoveFn = (direction: Direction, delta: number) => void;

export function onKeyboardMovement(move: MoveFn) {
  onKeyDown('ArrowUp', onMove('up'), { dedupe: false });
  onKeyDown('ArrowDown', onMove('down'), { dedupe: false });
  onKeyDown('ArrowLeft', onMove('left'), { dedupe: false });
  onKeyDown('ArrowRight', onMove('right'), { dedupe: false });

  function onMove(direction: Direction) {
    return function(e: KeyboardEvent) {
      let delta = 1;
      if (e.ctrlKey) delta = 5;
      if (e.shiftKey) delta = 10;
      if (e.ctrlKey && e.shiftKey) delta = 25;

      move(direction, delta);
    };
  }
}
