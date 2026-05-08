// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { get } from '../lib/api';
import { asyncRef } from '@tb-dev/vue';
import type { RemoteWorld } from '@tsukilabs/nil-bindings';

export function useWorlds() {
  const { state, loading } = asyncRef<readonly RemoteWorld[]>([], async () => {
    return get('get-remote-worlds').then((it) => it.json());
  });

  return {
    worlds: state as Readonly<typeof state>,
    loading,
  };
}
