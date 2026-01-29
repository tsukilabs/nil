// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { ref } from 'vue';
import { defineStore } from 'pinia';
import * as commands from '@/commands';

export const useUserStore = defineStore('user', () => {
  const authorizationToken = ref<Option<string>>(null);

  async function updateClient(options: Partial<ClientOptions> = {}) {
    await commands.updateClient({
      ...options,
      serverAddr: { kind: 'remote' },
      authorizationToken: authorizationToken.value,
    });
  }

  return {
    authorizationToken,
    updateClient,
  };
});
