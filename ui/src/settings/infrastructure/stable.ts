// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { defineStore } from 'pinia';
import { localRef } from '@tb-dev/vue';

export const useStableSettings = defineStore('stable-settings', () => {
  const hideUnmet = localRef('nil:stable-hide-unmet', true);

  return {
    hideUnmet,
  };
});
