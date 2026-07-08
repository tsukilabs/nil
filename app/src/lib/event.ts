// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import * as commands from "@/commands";
import { handleError } from "@/lib/error";
import { SavedataFile } from "@/core/savedata";
import type { Fn, MaybePromise, Option } from "@tb-dev/utils";
import { getCurrentWebviewWindow, type WebviewWindow } from "@tauri-apps/api/webviewWindow";
import type {
  ChatMessagePayload,
  CityPayload,
  DropPayload,
  EventPayload,
  MilitaryPayload,
  PlayerPayload,
  PublicCityPayload,
  ReportPayload,
  RoundPayload,
} from "@/types/event";

export type ListenerFn<T> = (payload: T) => MaybePromise<unknown>;

class Listener<T extends EventPayload> {
  public readonly on: (fn: ListenerFn<T>) => Promise<Fn>;

  private constructor(id: T["kind"]) {
    const name = `nil://${id}`;
    this.on = (fn: ListenerFn<T>) => {
      Listener.webview ??= getCurrentWebviewWindow();
      return Listener.webview.listen<T>(name, ({ payload }) => {
        (async () => {
          if (__DEBUG_ASSERTIONS__) {
            console.log(payload);
          }

          try {
            const world = NIL.world.getId();
            if (world && payload.world === world) {
              await fn(payload);
            }
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
    onChatMessage: new Listener<ChatMessagePayload>("chat-message"),
    onCity: new Listener<CityPayload>("city"),
    onDrop: new Listener<DropPayload>("drop"),
    onMilitary: new Listener<MilitaryPayload>("military"),
    onPlayer: new Listener<PlayerPayload>("player"),
    onPublicCity: new Listener<PublicCityPayload>("public-city"),
    onReport: new Listener<ReportPayload>("report"),
    onRound: new Listener<RoundPayload>("round"),
  } as const;
}

type EventObject = typeof Listener.listeners;

export type EventProxy = {
  [K in keyof EventObject]: EventObject[K]["on"];
};

export const events = new Proxy(Listener.listeners as unknown as EventProxy, {
  get: (target: EventProxy, key: keyof EventProxy) => {
    return Reflect.get(Reflect.get(target, key), "on");
  },

  defineProperty: () => false,
  deleteProperty: () => false,
  set: () => false,
});

export async function setDragDropEventListener() {
  const webview = getCurrentWebviewWindow();
  return webview.onDragDropEvent((event) => {
    if (event.payload.type === "drop" && event.payload.paths.length > 0) {
      onDragDropEvent(event.payload.paths).err();
    }
  });
}

async function onDragDropEvent(paths: readonly string[]) {
  for (const path of paths) {
    if (await commands.isSavedata(path)) {
      const savedata = await SavedataFile.read(path);
      await savedata.load();
      break;
    }
  }
}
