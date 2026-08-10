// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import * as commands from "@/commands";
import { wref } from "@/composables/cache/wref";

export function useCityLimit() {
  const cityLimit = wref("city-limit", 0, () => commands.getCityLimit());

  return {
    cityLimit: cityLimit.state,
    loading: cityLimit.loading,
    load: cityLimit.load,
  };
}
