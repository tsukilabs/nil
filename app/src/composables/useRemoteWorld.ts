// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { toRef, watch } from 'vue';
import * as commands from '@/commands';
import { asyncRef } from '@tb-dev/vue';
import { RemoteWorldImpl } from '@/core/model/remote-world';

export function useRemoteWorld(id: MaybeNilRef<WorldId>) {
  const idRef = toRef(id);
  const { state, isLoading, execute } = asyncRef(null, async () => {
    if (idRef.value) {
      const world = await commands.getRemoteWorld(idRef.value);
      return RemoteWorldImpl.create(world);
    }
    else {
      return null;
    }
  });

  watch(idRef, execute);

  return {
    remoteWorld: state,
    loading: isLoading,
    load: execute,
  };
}
