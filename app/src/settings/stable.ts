// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { app_StableSettings } from "@tsukilabs/nil-bindings";

export class StableSettingsImpl implements app_StableSettings {
  public hideUnmet = __CONSTS__.defaultSettings.stable.hideUnmet;
}
