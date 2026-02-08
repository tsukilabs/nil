// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import * as commands from '@/commands';
import { leaveGame } from '@/core/game';
import { handleError } from '@/lib/error';
import { lstat } from '@tauri-apps/plugin-fs';
import { extname } from '@tauri-apps/api/path';
import { go, QUERY_LOAD_LOCAL_GAME_PATH } from '@/router';
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
    onMilitaryUpdated: new this<MilitaryUpdatedPayload>('military-updated'),
    onPlayerUpdated: new this<PlayerUpdatedPayload>('player-updated'),
    onPublicCityUpdated: new this<PublicCityUpdatedPayload>('public-city-updated'),
    onReport: new this<ReportPayload>('report'),
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

export async function setDragDropEventListener() {
  const webview = getCurrentWebviewWindow();
  return webview.onDragDropEvent((event) => {
    if (event.payload.type === 'drop' && event.payload.paths.length > 0) {
      onDragDropEvent(event.payload.paths).err();
    }
  });
}

async function onDragDropEvent(paths: readonly string[]) {
  for (const path of paths) {
    const metadata = await lstat(path);
    if (metadata.isFile) {
      const extension = await extname(path);
      if (extension.toLowerCase() === 'nil') {
        await commands.allowScope(path);
        await leaveGame({ navigate: false });
        await go('load-local-game', {
          query: {
            [QUERY_LOAD_LOCAL_GAME_PATH]: path,
          },
        });
      }
    }
  }
}
