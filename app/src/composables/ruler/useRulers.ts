// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import * as commands from "@/commands";
import { wref } from "@/composables/cache/wref";

export function useRulers() {
  const rulers = wref("rulers", [], () => commands.getWorldRulers());

  return {
    rulers: rulers.state,
    loading: rulers.loading,
    load: rulers.load,
  };
}
