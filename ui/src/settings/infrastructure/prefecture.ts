// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { defineStore } from 'pinia';
import { localRef } from '@tb-dev/vue';

export const usePrefectureSettings = defineStore('prefecture-settings', () => {
  const hideMaxed = localRef('nil:prefecture-hide-maxed', false);
  const hideUnmet = localRef('nil:prefecture-hide-unmet', true);

  return {
    hideMaxed,
    hideUnmet,
  };
});
