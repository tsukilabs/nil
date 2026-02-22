// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

interface Settings {
  readonly academy: AcademySettings;
  readonly appearance: AppearanceSettings;
  readonly auth: AuthSettings;
  readonly general: GeneralSettings;
  readonly prefecture: PrefectureSettings;
  readonly stable: StableSettings;
  readonly workshop: WorkshopSettings;
}

interface AcademySettings {
  hideUnmet: boolean;
}

interface AppearanceSettings {
  colorMode: import('@vueuse/core').BasicColorSchema;
  theme: Theme;
}

interface AuthSettings {
  token: Option<string>;
}

interface GeneralSettings {
  autoUpdate: boolean;
  hideOnClose: boolean;
  locale: Locale;
}

interface PrefectureSettings {
  hideMaxed: boolean;
  hideUnmet: boolean;
}

interface StableSettings {
  hideUnmet: boolean;
}

interface WorkshopSettings {
  hideUnmet: boolean;
}

type Theme =
  | 'blue'
  | 'gray'
  | 'green'
  | 'neutral'
  | 'orange'
  | 'rose'
  | 'slate'
  | 'stone'
  | 'violet'
  | 'yellow'
  | 'zinc';
