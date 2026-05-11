// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { get } from "../lib/api";
import { asyncRef } from "@tb-dev/vue";

export function useServerVersion() {
  const { state, loading } = asyncRef(null, async () => {
    const response = await get("version");
    const version = await response.text();
    return version;
  });

  return {
    version: state as Readonly<typeof state>,
    loading,
  };
}
