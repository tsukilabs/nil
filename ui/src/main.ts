// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import 'vue-sonner/style.css';
import '@/assets/base.css';
import '@/assets/fonts.css';
import '@/assets/nsr.css';
import '@tb-dev/vue-components/style';
import '@/lib/prototype';
import App from '@/App.vue';
import { createApp } from 'vue';
import { i18n } from '@/locale';
import { router } from '@/router';
import { handleError } from '@/lib/error';
import { initEntities } from '@/core/entity';
import { registerGlobalComponents } from '@/components';
import { setCurrentApp, setErrorHandler } from '@tb-dev/vue';

const app = createApp(App);

setCurrentApp(app);
setErrorHandler(handleError, app);
registerGlobalComponents(app);

app.use(i18n());
app.use(router);

router
  .push({ name: 'home' })
  .then(() => initEntities())
  .then(() => app.mount('#app'))
  .err();
