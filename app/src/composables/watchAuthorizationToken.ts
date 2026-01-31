// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { go } from '@/router';
import { storeToRefs } from 'pinia';
import * as commands from '@/commands';
import { handleError } from '@/lib/error';
import { useUserStore } from '@/stores/user';
import { tryOnScopeDispose, watchImmediate } from '@vueuse/core';

export function watchAuthorizationToken() {
  const userStore = useUserStore();
  const { authorizationToken } = storeToRefs(userStore);
  const { isAuthorizationTokenValid } = userStore;

  const stop = watchImmediate(authorizationToken, async () => {
    try {
      if (!(await isAuthorizationTokenValid())) {
        await go('home');
        await commands.clearAllBrowsingData();
      }
    }
    catch (err) {
      handleError(err);
    }
  });

  tryOnScopeDispose(() => stop());
}
