// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { get } from '../lib/api';
import { asyncRef } from '@tb-dev/vue';

export interface World {
  readonly currentRound: number;
  readonly totalPlayers: number;
  readonly config: {
    readonly id: string;
    readonly name: string;
  };
}

export function useWorlds() {
  const { state, loading } = asyncRef<readonly World[]>([], async () => {
    return get('get-remote-worlds').then((it) => it.json());
  });

  return {
    worlds: state as Readonly<typeof state>,
    loading,
  };
}
