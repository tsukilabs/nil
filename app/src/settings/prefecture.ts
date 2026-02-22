// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

export class PrefectureSettingsImpl implements PrefectureSettings {
  public hideMaxed = __CONSTS__.defaultSettings.prefecture.hideMaxed;
  public hideUnmet = __CONSTS__.defaultSettings.prefecture.hideUnmet;
}
