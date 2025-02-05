import { type InjectionKey, inject as originalInject } from 'vue';

export function app() {
  return window.__NIL__.app;
}

export function runWithContext<T>(fn: () => T) {
  return app().runWithContext(fn);
}

export function provide<T>(key: InjectionKey<T> | string, value: T) {
  app().provide(key, value);
}

export function inject<T>(key: InjectionKey<T>): T {
  const value = tryInject(key);
  if (typeof value === 'undefined') {
    throw new TypeError('injection failed: value was not provided');
  }

  return value;
}

export function tryInject<T>(key: InjectionKey<T>) {
  return app().runWithContext(() => originalInject(key));
}
