import type * as $ from './payload';
import { handleError } from '@/lib/error';
import type { Fn, MaybePromise, Option } from '@tb-dev/utils';
import { getCurrentWebviewWindow, type WebviewWindow } from '@tauri-apps/api/webviewWindow';

export type ListenerFn<T> = (payload: T) => MaybePromise<unknown>;

class Listener<T> {
  private static webview: Option<WebviewWindow>;

  public readonly on: (fn: ListenerFn<T>) => Promise<Fn>;

  private constructor(id: string) {
    this.on = (fn: ListenerFn<T>) => {
      Listener.webview ??= getCurrentWebviewWindow();
      return Listener.webview.listen<T>(id, ({ payload }) => {
        (async () => {
          try {
            await fn(payload);
          } catch (err) {
            handleError(err);
          }
        })();
      });
    };
  }

  public static create() {
    return {
      onPlayerJoined: new this<$.PlayerJoinedPayload>('player-joined'),
      onRoundUpdated: new this<$.RoundUpdatedPayload>('round-updated'),
    };
  }
}

type EventObject = ReturnType<typeof Listener.create>;

export type EventProxy = {
  [K in keyof EventObject]: EventObject[K]['on'];
};

export const events = new Proxy(Listener.create() as unknown as EventProxy, {
  get: (target: EventProxy, key: keyof EventProxy) => {
    return Reflect.get(Reflect.get(target, key), 'on');
  },

  defineProperty: () => false,
  deleteProperty: () => false,
  set: () => false,
});
