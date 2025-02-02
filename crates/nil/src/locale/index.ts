import enUS from './en-US.json';
import ptBR from './pt-BR.json';
import { useI18n } from 'vue-i18n';

export type Locale = keyof typeof locales;
export type LocaleSchema = typeof enUS;

export const locales = {
  'en-US': enUS,
  'pt-BR': ptBR,
};

export function useLocale() {
  return useI18n<{ message: LocaleSchema }>({
    useScope: 'global',
  });
}
