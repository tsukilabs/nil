// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { BasicColorSchema } from '@vueuse/core';
import type { AppearanceSettings } from '@/types/settings';

export class AppearanceSettingsImpl implements AppearanceSettings {
  public colorMode: BasicColorSchema = 'dark';
}
