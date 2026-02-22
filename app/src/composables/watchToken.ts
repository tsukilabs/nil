// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { go } from '@/router';
import { computed } from 'vue';
import { handleError } from '@/lib/error';
import { useSettings } from '@/stores/settings';
import { tryOnScopeDispose, watchImmediate } from '@vueuse/core';

export function watchToken() {
  const settings = useSettings();
  const token = computed(() => settings.auth.token);

  const stop = watchImmediate(token, async () => {
    try {
      if (!(await settings.auth.isTokenValid())) {
        await go('home');
      }
    }
    catch (err) {
      handleError(err);
    }
  });

  tryOnScopeDispose(() => stop());
}
