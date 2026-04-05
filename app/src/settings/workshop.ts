// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { WorkshopSettings } from '@/types/settings';

export class WorkshopSettingsImpl implements WorkshopSettings {
  public hideUnmet = __CONSTS__.defaultSettings.workshop.hideUnmet;
}
