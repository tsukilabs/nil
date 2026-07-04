// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { get } from "../lib/api";
import { compare } from "../lib/intl";
import { inBrowser } from "vitepress";
import { asyncRef } from "@tb-dev/vue";
import type { RemoteWorld } from "@tsukilabs/nil-bindings";

export function useWorlds() {
  const { state, loading } = asyncRef<readonly RemoteWorld[]>([], async () => {
    if (inBrowser) {
      const response = await get("get-remote-worlds");
      const worlds: RemoteWorld[] = await response.json();
      return worlds.toSorted((a, b) => compare(a.config.name, b.config.name));
    }
    else {
      return [];
    }
  });

  return {
    worlds: state as Readonly<typeof state>,
    loading,
  };
}
