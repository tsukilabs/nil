// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

export class GeneralSettingsImpl implements GeneralSettings {
  public autoUpdate = __CONSTS__.defaultSettings.general.autoUpdate;
  public hideOnClose = __CONSTS__.defaultSettings.general.hideOnClose;
  public locale = getDefaultLocale();
}

function getDefaultLocale(): Locale {
  if (window.navigator.language.startsWith('pt')) {
    return 'pt-BR';
  }
  else {
    return __CONSTS__.defaultSettings.general.locale;
  }
}
