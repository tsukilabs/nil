// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { StableSettings } from '@/types/settings';

export class StableSettingsImpl implements StableSettings {
  public hideUnmet = __CONSTS__.defaultSettings.stable.hideUnmet;
}
