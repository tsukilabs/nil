// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { PrefectureSettings } from '@/types/settings';

export class PrefectureSettingsImpl implements PrefectureSettings {
  public hideMaxed = __CONSTS__.defaultSettings.prefecture.hideMaxed;
  public hideUnmet = __CONSTS__.defaultSettings.prefecture.hideUnmet;
}
