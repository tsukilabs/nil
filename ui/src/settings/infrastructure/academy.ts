// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { ref } from 'vue';
import { defineStore } from 'pinia';

export const useAcademySettings = defineStore('academy', () => {
  const hideUnmet = ref(true);

  return {
    hideUnmet,
  };
});
