// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { asyncRef } from '@tb-dev/vue';
import { computed, toRef, watch } from 'vue';
import { PublicBotImpl } from '@/core/model/npc/bot/public-bot';

export function usePublicBot(id: MaybeNilRef<BotId>) {
  const idRef = toRef(id);
  const { state, isLoading, execute } = asyncRef(null, async () => {
    return idRef.value ? PublicBotImpl.load(idRef.value) : null;
  });

  const coords = computed(() => state.value?.coords ?? []);

  watch(idRef, execute);

  return {
    bot: state,
    coords,
    loading: isLoading,
    load: execute,
  };
}
