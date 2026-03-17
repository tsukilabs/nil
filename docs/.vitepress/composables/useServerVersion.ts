// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { get } from '../lib/api';
import { asyncRef } from '@tb-dev/vue';

export function useServerVersion() {
  const { state, loading } = asyncRef(null, async () => {
    return get('version').then((it) => it.text());
  });

  return {
    version: state as Readonly<typeof state>,
    loading,
  };
}
