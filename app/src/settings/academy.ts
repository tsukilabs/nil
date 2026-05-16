// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { app_AcademySettings } from "@tsukilabs/nil-bindings";

export class AcademySettingsImpl implements app_AcademySettings {
  public hideUnmet = __CONSTS__.defaultSettings.academy.hideUnmet;
}
