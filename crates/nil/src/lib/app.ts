import { type InjectionKey, inject as originalInject } from 'vue';

export function app() {
  return window.__NIL__.app;
}

export function runWithContext<T>(fn: () => T) {
  return window.__NIL__.app.runWithContext(fn);
}

export function provide<T>(key: InjectionKey<T> | string, value: T) {
  window.__NIL__.app.provide(key, value);
}

export function inject<T>(key: InjectionKey<T>): T {
  const value = fallibleInject(key);
  if (typeof value === 'undefined') {
    throw new TypeError('injection failed: value was not provided');
  }

  return value;
}

export function fallibleInject<T>(key: InjectionKey<T>) {
  return window.__NIL__.app.runWithContext(() => originalInject(key));
}
