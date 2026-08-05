//! Copyright (C) Call of Nil contributors
//! SPDX-License-Identifier: AGPL-3.0-only

import enUS from "./locale/en-US.json";
import ptBR from "./locale/pt-BR.json";
import type { Locale } from "@tsukilabs/nil-bindings";
import { createI18n, useI18n as use } from "vue-i18n";

export function i18n() {
  return createI18n<[typeof enUS], Locale>({
    fallbackLocale: ["en-US", "pt-BR"],
    legacy: false,
    locale: "en-US",
    fallbackWarn: false,
    missingWarn: false,
    messages: {
      "en-US": enUS,
      "pt-BR": ptBR,
    },
  });
}

export function useI18n() {
  return use<[typeof enUS], Locale>();
}

export { enUS, ptBR };
