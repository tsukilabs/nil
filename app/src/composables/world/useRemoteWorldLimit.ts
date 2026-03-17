// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import * as commands from '@/commands';
import { localRef } from '@tb-dev/vue';
import { handleError } from '@/lib/error';
import type { DeepReadonly, Ref } from 'vue';

interface Limit {
  readonly version: string;
  global: Option<number>;
  perUser: Option<number>;
}

export function useRemoteWorldLimit() {
  const limit = localRef('remote-world-limit', defaultLimit(), {
    initOnMounted: false,
    mergeDefaults: true,
    writeDefaults: true,
  });

  getRemoteWorldLimit(limit).catch(handleError);

  return limit as DeepReadonly<typeof limit>;
}

async function getRemoteWorldLimit(limit: Ref<Limit>) {
  if (await commands.isRemote()) {
    if (!isValidLimit(limit.value.global, limit.value.version)) {
      limit.value.global = await commands.getRemoteWorldLimit();
    }

    if (!isValidLimit(limit.value.perUser, limit.value.version)) {
      limit.value.perUser = await commands.getRemoteWorldLimitPerUser();
    }
  }
}

function isValidLimit(limit: Option<number>, version: string) {
  return (
    version === __VERSION__ &&
    typeof limit === 'number' &&
    limit > 0
  );
}

function defaultLimit(): Limit {
  return {
    version: __VERSION__,
    global: null,
    perUser: null,
  };
}
