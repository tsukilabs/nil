// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { defineStore } from 'pinia';
import { localRef } from '@tb-dev/vue';

export const useAcademySettings = defineStore('academy-settings', () => {
  const hideUnmet = localRef('nil:academy-hide-unmet', true);

  return {
    hideUnmet,
  };
});
