// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { computed } from 'vue';
import { toPlayerRef } from '@/composables/util/toRef';
import type { PlayerImpl } from '@/core/model/player/player';

export function usePlayerStorageCapacity(player?: MaybeNilRef<PlayerImpl>) {
  const playerRef = toPlayerRef(player);
  return computed(() => playerRef.value?.capacity);
}
