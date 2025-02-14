import '@/assets/index.css';
import '@/lib/prototype';
import App from '@/App.vue';
import { createApp } from 'vue';
import { go, router } from '@/router';
import { createI18n } from 'vue-i18n';
import * as commands from '@/commands';
import { handleError } from '@/lib/error';
import { type Locale, locales, type LocaleSchema } from './locale';

const app = createApp(App);

Object.defineProperty(window, '__NIL__', {
  configurable: false,
  enumerable: true,
  value: Object.freeze({ app }),
  writable: false,
});

app.config.globalProperties.$go = go;
app.config.globalProperties.$c = commands;
app.config.errorHandler = (err) => handleError(err);

app.config.warnHandler = (message, _, trace) => {
  if (!message.includes('"Symbol(game)" not found')) {
    console.warn(`[Vue warn]: ${message}\n${trace}`);
  }
};

const i18n = createI18n<[LocaleSchema], Locale>({
  fallbackLocale: ['en-US', 'pt-BR'],
  legacy: false,
  locale: 'en-US',
  messages: locales,
});

app.use(i18n);
app.use(router);

router
  .push({ name: 'home' })
  .then(() => app.mount('#app'))
  .handleError();
