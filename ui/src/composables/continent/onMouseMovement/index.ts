// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { toRef } from 'vue';
import { clamp } from 'es-toolkit';
import type { Fn } from '@tb-dev/utils';
import type { Continent } from 'nil-continent';
import { tryOnScopeDispose, useEventListener } from '@vueuse/core';

export function onMouseMovement(
  container: MaybeNilRef<HTMLElement>,
  continent: Continent,
  render: Fn,
) {
  const el = toRef(container);
  const max = NIL.world.refs().continentSize.value - 1;

  let active = false;
  let initialX = 0;
  let initialY = 0;
  const centerX = 0;
  const centerY = 0;

  useEventListener(el, 'touchstart', onStart);
  useEventListener(el, 'touchmove', onMove);
  useEventListener(el, 'touchend', onEnd);

  useEventListener(el, 'mousedown', onStart);
  useEventListener(el, 'mousemove', onMove);
  useEventListener(el, 'mouseup', onEnd);

  tryOnScopeDispose(() => {
    active = false;
  });

  function onStart(e: TouchEvent | MouseEvent) {
    if (!active) {
      if ('touches' in e) {
        initialX = e.touches[0].clientX;
        initialY = e.touches[0].clientY;
      }
      else {
        initialX = e.clientX;
        initialY = e.clientY;
      }

      active = true;
    }
  }

  function onMove(e: TouchEvent | MouseEvent) {
    if (active) {
      e.preventDefault();
      el.value?.classList.add('dragging');

      let deltaX: number;
      let deltaY: number;
      if ('touches' in e) {
        deltaX = e.touches[0].clientX - initialX;
        deltaY = e.touches[0].clientY - initialY;
      }
      else {
        deltaX = e.clientX - initialX;
        deltaY = e.clientY - initialY;
      }

      const x = clamp(centerX + deltaX, 0, max);
      const y = clamp(centerY + deltaY, 0, max);

      if (x !== centerX || y !== centerY) {
        continent.set_center(x, y);
        render();
      }
    }
  }

  function onEnd(e: TouchEvent | MouseEvent) {
    e.preventDefault();
    active = false;
    el.value?.classList.remove('dragging');
  }
}
