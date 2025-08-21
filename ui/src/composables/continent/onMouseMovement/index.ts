// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { clamp } from 'es-toolkit';
import { type Ref, toRef } from 'vue';
import type { Fn } from '@tb-dev/utils';
import type { Continent } from 'nil-continent';
import { tryOnScopeDispose, useEventListener } from '@vueuse/core';

export function onMouseMovement(
  continent: Continent,
  render: Fn,
  container: MaybeNilRef<HTMLElement>,
  cellSize: Readonly<Ref<number>>,
) {
  const el = toRef(container);
  const max = NIL.world.refs().continentSize.value - 1;

  let active = false;
  let clientX = 0;
  let clientY = 0;

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
      setClientCoords(e);
      active = true;
    }
  }

  function onMove(e: TouchEvent | MouseEvent) {
    if (active) {
      e.preventDefault();

      let deltaX: number;
      let deltaY: number;
      if ('touches' in e) {
        deltaX = -(e.touches[0].clientX - clientX);
        deltaY = -(e.touches[0].clientY - clientY);
      }
      else {
        deltaX = -(e.clientX - clientX);
        deltaY = -(e.clientY - clientY);
      }

      setClientCoords(e);

      const center = continent.center();
      const centerX = center.x();
      const centerY = center.y();

      const x = clamp(centerX + Math.floor(deltaX / cellSize.value), 0, max);
      const y = clamp(centerY + Math.floor(deltaY / cellSize.value), 0, max);

      if (x !== centerX || y !== centerY) {
        continent.set_center(x, y);
        render();
      }
    }
  }

  function onEnd(e: TouchEvent | MouseEvent) {
    e.preventDefault();
    active = false;
  }

  function setClientCoords(e: TouchEvent | MouseEvent) {
    if ('touches' in e) {
      clientX = e.touches[0].clientX;
      clientY = e.touches[0].clientY;
    }
    else {
      clientX = e.clientX;
      clientY = e.clientY;
    }
  }
}
