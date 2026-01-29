// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import * as commands from '@/commands';
import { asyncRef } from '@tb-dev/vue';
import { RemoteWorldImpl } from '@/core/model/remote-world';

export function useRemoteWorlds() {
  const { state, isLoading, execute } = asyncRef<readonly RemoteWorldImpl[]>([], async () => {
    const worlds = await commands.getRemoteWorlds();
    return worlds.map(RemoteWorldImpl.create.bind(RemoteWorldImpl));
  });

  return {
    remoteWorlds: state,
    loading: isLoading,
    load: execute,
  };
}
