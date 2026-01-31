// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { go } from '@/router';
import { storeToRefs } from 'pinia';
import { useUserStore } from '@/stores/user';
import { clearAllBrowsingData } from '@/lib/webview';
import { tryOnScopeDispose, watchImmediate } from '@vueuse/core';

export function watchAuthorizationToken(fallbackScene: Scene) {
  const userStore = useUserStore();
  const { authorizationToken } = storeToRefs(userStore);
  const { isAuthorizationTokenValid } = userStore;

  const stop = watchImmediate(authorizationToken, async () => {
    if (!(await isAuthorizationTokenValid())) {
      await go(fallbackScene);
      await clearAllBrowsingData();
    }
  });

  tryOnScopeDispose(() => stop());
}
