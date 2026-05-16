// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { BasicColorSchema } from "@vueuse/core";
import type { app_AppearanceSettings } from "@tsukilabs/nil-bindings";

export class AppearanceSettingsImpl implements app_AppearanceSettings {
  public colorMode: BasicColorSchema = "dark";
}
