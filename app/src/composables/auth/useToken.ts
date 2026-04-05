// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { computed } from 'vue';
import * as commands from '@/commands';
import { asyncComputed } from '@tb-dev/vue';
import { useSettings } from '@/stores/settings';
import type { PlayerId } from '@/types/core/player';

export function useToken() {
  const settings = useSettings();
  const token = computed(() => settings.auth.token);

  const playerId = asyncComputed<Option<PlayerId>>(null, () => {
    return commands.validateToken(token.value);
  });

  const isTokenValid = computed(() => Boolean(playerId.value));

  return {
    token,
    playerId,
    isTokenValid,
  };
}
