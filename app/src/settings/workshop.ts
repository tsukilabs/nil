// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { app_WorkshopSettings } from "@tsukilabs/nil-bindings";

export class WorkshopSettingsImpl implements app_WorkshopSettings {
  public hideUnmet = __CONSTS__.defaultSettings.workshop.hideUnmet;
}
