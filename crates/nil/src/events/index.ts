import type * as $ from './payload';
import type { Fn, Option } from '@tb-dev/utils';
import { getCurrentWebviewWindow, type WebviewWindow } from '@tauri-apps/api/webviewWindow';

type ListenerFn<T> = (payload: T) => void;

class Listener<T> {
  private static webview: Option<WebviewWindow>;

  public readonly on: (fn: ListenerFn<T>) => Promise<Fn>;

  private constructor(id: string) {
    this.on = (fn: ListenerFn<T>) => {
      Listener.webview ??= getCurrentWebviewWindow();
      return Listener.webview.listen<T>(id, ({ payload }) => {
        fn(payload);
      });
    };
  }

  public static create() {
    return {
      onPlayerJoined: new this<$.PlayerJoinedPayload>('player-joined'),
    };
  }
}

type EventObject = ReturnType<typeof Listener.create>;

export const events = new Proxy(
  Listener.create() as unknown as {
    [K in keyof EventObject]: EventObject[K]['on'];
  },
  {
    // @ts-expect-error TODO: this should be properly typed.
    get: (target: EventObject, key: keyof EventObject) => {
      return Reflect.get(Reflect.get(target, key), 'on');
    },

    defineProperty: () => false,
    deleteProperty: () => false,
    set: () => false,
  }
);
