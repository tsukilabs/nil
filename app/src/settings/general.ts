// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { app_GeneralSettings, Locale } from "@tsukilabs/nil-bindings";

export class GeneralSettingsImpl implements app_GeneralSettings {
  public autoUpdate = __CONSTS__.defaultSettings.general.autoUpdate;
  public hideOnClose = __CONSTS__.defaultSettings.general.hideOnClose;
  public locale = getDefaultLocale();
}

function getDefaultLocale(): Locale {
  if (window.navigator.language.startsWith("pt")) {
    return "pt-BR";
  }
  else {
    return __CONSTS__.defaultSettings.general.locale;
  }
}
