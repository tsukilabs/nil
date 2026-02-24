// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import * as commands from '@/commands';
import { asyncRef } from '@tb-dev/vue';
import { ScriptImpl } from '@/core/model/scripts/script';

export function useScripts() {
  // TODO: I don’t think it makes any sense to eagerly load all scripts like this.
  // We should only read the dir entries, then load the file contents on demand later.
  const { state, loading, load } = asyncRef<readonly ScriptImpl[]>([], async () => {
    const scripts = await commands.loadScripts();
    return scripts.map((script) => ScriptImpl.create(script));
  });

  return {
    scripts: state as Readonly<typeof state>,
    loading,
    load,
  };
}
