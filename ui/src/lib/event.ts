// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { handleError } from '@/lib/error';
import type { Fn, MaybePromise, Option } from '@tb-dev/utils';
import { getCurrentWebviewWindow, type WebviewWindow } from '@tauri-apps/api/webviewWindow';

export type ListenerFn<T> = (payload: T) => MaybePromise<unknown>;

class Listener<T extends EventPayload> {
  public readonly on: (fn: ListenerFn<T>) => Promise<Fn>;

  private constructor(id: T['kind']) {
    const name = `nil://${id}`;
    this.on = (fn: ListenerFn<T>) => {
      Listener.webview ??= getCurrentWebviewWindow();
      return Listener.webview.listen<T>(name, ({ payload }) => {
        (async () => {
          if (__DEBUG_ASSERTIONS__) {
            console.log(payload);
          }

          try {
            await fn(payload);
          }
          catch (err) {
            handleError(err);
          }
        })();
      });
    };
  }

  private static webview: Option<WebviewWindow>;

  public static readonly listeners = {
    onChatUpdated: new this<ChatUpdatedPayload>('chat-updated'),
    onCityUpdated: new this<CityUpdatedPayload>('city-updated'),
    onPlayerUpdated: new this<PlayerUpdatedPayload>('player-updated'),
    onPublicCityUpdated: new this<PublicCityUpdatedPayload>('public-city-updated'),
    onRoundUpdated: new this<RoundUpdatedPayload>('round-updated'),
  } as const;
}

type EventObject = typeof Listener.listeners;

export type EventProxy = {
  [K in keyof EventObject]: EventObject[K]['on'];
};

export const events = new Proxy(Listener.listeners as unknown as EventProxy, {
  get: (target: EventProxy, key: keyof EventProxy) => {
    return Reflect.get(Reflect.get(target, key), 'on');
  },

  defineProperty: () => false,
  deleteProperty: () => false,
  set: () => false,
});
