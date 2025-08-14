// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { ref } from 'vue';
import { defineStore } from 'pinia';

export const usePrefectureSettings = defineStore('prefecture', () => {
  const hideMaxed = ref(false);
  const hideUnmet = ref(true);

  return {
    hideMaxed,
    hideUnmet,
  };
});
