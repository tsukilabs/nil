// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { AcademySettings } from '@/types/settings';

export class AcademySettingsImpl implements AcademySettings {
  public hideUnmet = __CONSTS__.defaultSettings.academy.hideUnmet;
}
