// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { go } from '@/router';
import { storeToRefs } from 'pinia';
import { useUserStore } from '@/stores/user';
import { validateToken } from '@/commands/server';
import { tryOnUnmounted, watchImmediate } from '@vueuse/core';

export function watchToken(fallbackScene: Scene) {
  const userStore = useUserStore();
  const { authorizationToken } = storeToRefs(userStore);

  const stop = watchImmediate(authorizationToken, async (token) => {
    if (!token || !(await validateToken(token))) {
      await go(fallbackScene);
    }
  });

  tryOnUnmounted(() => stop());
}
