// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { get } from "../lib/api";
import { inBrowser } from "vitepress";
import { asyncRef } from "@tb-dev/vue";

export function useServerVersion() {
  const { state, loading } = asyncRef(null, async () => {
    if (inBrowser) {
      const response = await get("version");
      const version: string = await response.json();
      return version;
    }
    else {
      return null;
    }
  });

  return {
    version: state as Readonly<typeof state>,
    loading,
  };
}
