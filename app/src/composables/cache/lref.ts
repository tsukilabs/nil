// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { localRef } from "@tb-dev/vue";
import { prependKey } from "@/lib/storage";
import type { UseStorageOptions } from "@vueuse/core";

export function lref<T>(key: string, defaultValue: T, options?: UseStorageOptions<T>) {
  key = prependKey(key);
  return localRef(key, defaultValue, options);
}
