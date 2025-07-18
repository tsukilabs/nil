// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import 'vue-sonner/style.css';
import '@/assets/style/base.css';
import '@/assets/style/vars.css';
import '@/assets/style/themes.css';
import '@/assets/style/fonts.css';
import '@/assets/style/main.css';
import '@/assets/style/nsr.css';
import '@/lib/prototype';
import App from '@/App.vue';
import { createApp } from 'vue';
import { i18n } from '@/locale';
import { router } from '@/router';
import { createPinia } from 'pinia';
import { handleError } from '@/lib/error';
import { initEntities } from '@/core/entity';
import { registerGlobalComponents } from '@/components';
import { setCurrentApp, setErrorHandler } from '@tb-dev/vue';

const app = createApp(App);
const pinia = createPinia();

setCurrentApp(app);
setErrorHandler(handleError, app);
registerGlobalComponents(app);

app.use(i18n());
app.use(router);
app.use(pinia);

router
  .push({ name: 'home' })
  .then(() => initEntities())
  .then(() => app.mount('#app'))
  .err();
