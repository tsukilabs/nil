// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { ref } from 'vue';
import { defineStore } from 'pinia';

export const useStableSettings = defineStore('stable', () => {
  const hideUnmet = ref(true);

  return {
    hideUnmet,
  };
});
