// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { getCurrentScope } from "vue";
import { useRoute } from "vue-router";
import { runWithContext } from "@tb-dev/vue";

export function prependKey(key: string): string {
  if (getCurrentScope()) {
    return runWithContext(() => {
      const route = useRoute();
      if (route.name && typeof route.name === "string") {
        return `nil:${route.name}:${key}`;
      }
      else {
        return `nil:${key}`;
      }
    });
  }
  else {
    return key;
  }
}
