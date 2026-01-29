// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { ref } from 'vue';
import { defineStore } from 'pinia';
import * as commands from '@/commands';

export const useUserStore = defineStore('user', () => {
  const authorizationToken = ref<Option<string>>(null);

  async function validateToken() {
    if (authorizationToken.value && await commands.isRemote()) {
      const player = await commands.validateToken(authorizationToken.value);
      if (!player) {
        authorizationToken.value = null;
      }
    }
  }

  return {
    authorizationToken,
    validateToken,
  };
});
