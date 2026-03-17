// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import * as commands from '@/commands';
import { sessionRef } from '@tb-dev/vue';
import { readonly, type Ref } from 'vue';
import { handleError } from '@/lib/error';

export function useRemoteWorldLimit() {
  const worldLimit = sessionRef('remote-world-limit', 0, {
    initOnMounted: false,
    writeDefaults: true,
  });

  getRemoteWorldLimit(worldLimit).catch(handleError);

  return readonly(worldLimit);
}

async function getRemoteWorldLimit(limit: Ref<number>) {
  if (limit.value === 0 && await commands.isRemote()) {
    limit.value = await commands.getRemoteWorldLimit();
  }
}
