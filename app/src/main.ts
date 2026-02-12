// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import '@tb-dev/vue-sonner/style.css';
import '@/assets/style/base.css';
import '@/assets/style/vars.css';
import '@/assets/style/themes.css';
import '@/assets/style/fonts.css';
import '@/assets/style/main.css';
import '@/assets/style/layout.css';
import '@/lib/prototype';
import App from '@/App.vue';
import { i18n } from '@/locale';
import { createApp } from 'vue';
import { router } from '@/router';
import { createPinia } from 'pinia';
import { handleError } from '@/lib/error';
import { initEntities } from '@/core/entity';
import { TauriPluginPinia } from '@tauri-store/pinia';
import { registerGlobalComponents } from '@/lib/global';
import { setCurrentApp, setErrorHandler } from '@tb-dev/vue';

const app = createApp(App);
const pinia = createPinia();

setCurrentApp(app);
setErrorHandler(handleError, app);
registerGlobalComponents(app);

pinia.use(TauriPluginPinia({
  autoStart: true,
  saveOnChange: true,
  saveStrategy: 'debounce',
  saveInterval: 1000,
  hooks: { error: handleError },
}));

app.use(i18n());
app.use(router);
app.use(pinia);

initEntities();

app.mount('#app');
