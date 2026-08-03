// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import * as commands from "@/commands";
import { asyncRef } from "@tb-dev/vue";

export function useRulers() {
  const rulers = asyncRef([], () => commands.getWorldRulers());

  return {
    rulers: rulers.state,
    loading: rulers.loading,
    load: rulers.load,
  };
}
