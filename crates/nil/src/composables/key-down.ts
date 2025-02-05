import { handleError } from '@/lib/error';
import type { MaybePromise, Option } from '@tb-dev/utils';
import {
  type KeyFilter,
  onKeyStroke,
  type OnKeyStrokeOptions,
  tryOnScopeDispose,
} from '@vueuse/core';

export type KeyDownEventHandler = Option<(event: KeyboardEvent) => MaybePromise<unknown>>;

export interface OnKeyDownOptions extends Omit<OnKeyStrokeOptions, 'eventName'> {
  altKey?: boolean;
  ctrlKey?: boolean;
  metaKey?: boolean;
  prevent?: boolean;
  shiftKey?: boolean;
}

export function onKeyDown(
  key: KeyFilter,
  handler?: KeyDownEventHandler,
  options: OnKeyDownOptions = {}
) {
  const {
    altKey = false,
    ctrlKey = false,
    metaKey = false,
    prevent = true,
    shiftKey = false,
  } = options;

  const callback = (e: KeyboardEvent) => {
    if (
      e.altKey !== altKey ||
      e.ctrlKey !== ctrlKey ||
      e.metaKey !== metaKey ||
      e.shiftKey !== shiftKey
    ) {
      return;
    }

    if (prevent) {
      e.preventDefault();
    }

    (async () => {
      try {
        await handler?.(e);
      } catch (err) {
        handleError(err);
      }
    })();
  };

  const stop = onKeyStroke(key, callback, {
    ...options,
    dedupe: true,
    eventName: 'keydown',
  });

  tryOnScopeDispose(() => stop());

  return stop;
}

export function onEnterKey(handler: KeyDownEventHandler, options: OnKeyDownOptions = {}) {
  return onKeyDown('Enter', handler, {
    ...options,
    altKey: false,
    ctrlKey: false,
    shiftKey: false,
  });
}

export function onAltKeyDown(
  key: KeyFilter,
  handler?: KeyDownEventHandler,
  options?: Omit<OnKeyDownOptions, 'altKey' | 'eventName'>
) {
  return onKeyDown(key, handler, { ...options, altKey: true });
}

export function onCtrlKeyDown(
  key: KeyFilter,
  handler?: KeyDownEventHandler,
  options?: Omit<OnKeyDownOptions, 'ctrlKey' | 'eventName'>
) {
  return onKeyDown(key, handler, { ...options, ctrlKey: true });
}
