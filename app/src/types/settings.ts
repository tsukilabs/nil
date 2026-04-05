// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

export interface Settings {
  readonly academy: AcademySettings;
  readonly appearance: AppearanceSettings;
  readonly auth: AuthSettings;
  readonly general: GeneralSettings;
  readonly prefecture: PrefectureSettings;
  readonly stable: StableSettings;
  readonly workshop: WorkshopSettings;
}

export interface AcademySettings {
  hideUnmet: boolean;
}

export interface AppearanceSettings {
  colorMode: import('@vueuse/core').BasicColorSchema;
}

export interface AuthSettings {
  token: Option<string>;
}

export interface GeneralSettings {
  autoUpdate: boolean;
  hideOnClose: boolean;
  locale: Locale;
}

export interface PrefectureSettings {
  hideMaxed: boolean;
  hideUnmet: boolean;
}

export interface StableSettings {
  hideUnmet: boolean;
}

export interface WorkshopSettings {
  hideUnmet: boolean;
}
