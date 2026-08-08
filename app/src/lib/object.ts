// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { isPlainObject } from "es-toolkit/predicate";

export function freezeObject(object: Record<PropertyKey, unknown>) {
  Object.freeze(object);

  for (const key of Reflect.ownKeys(object)) {
    const value = object[key];
    if (isPlainObject(value)) {
      freezeObject(value);
    }
  }
}
