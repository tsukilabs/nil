// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import * as commands from '@/commands';
import { asyncRef } from '@tb-dev/vue';
import { SocketAddrV4 } from '@/lib/net/addr-v4';

export function useLocalServerAddr() {
  const { state, execute, isLoading } = asyncRef(null, async () => {
    const serverAddr = await commands.getServerAddr();
    if (serverAddr.kind === 'local') {
      return SocketAddrV4.parse(serverAddr.addr);
    }
    else {
      return null;
    }
  });

  return {
    serverAddr: state,
    loading: isLoading,
    load: execute,
  };
}
