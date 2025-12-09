// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { createI18n } from 'vue-i18n';
import enUS from './en-US/global.json';
import ptBR from './pt-BR/global.json';

export function i18n() {
  return createI18n<[typeof enUS], Locale>({
    fallbackLocale: ['en-US', 'pt-BR'],
    legacy: false,
    locale: 'en-US',
    fallbackWarn: false,
    missingWarn: false,
    messages: {
      'en-US': enUS,
      'pt-BR': ptBR,
    },
  });
}
