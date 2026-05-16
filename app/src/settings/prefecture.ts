// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { app_PrefectureSettings } from "@tsukilabs/nil-bindings";

export class PrefectureSettingsImpl implements app_PrefectureSettings {
  public hideMaxed = __CONSTS__.defaultSettings.prefecture.hideMaxed;
  public hideUnmet = __CONSTS__.defaultSettings.prefecture.hideUnmet;
}
